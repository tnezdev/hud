from pathlib import Path
import json,gzip,hashlib,subprocess,collections,difflib
R=Path('/home/tnez/Work/tnezdev/hud'); E=R/'docs/assets/authoring-fidelity/run-1'; O=Path('/tmp/hud-fidelity-run1-rS7MUp/review')
log=[]
for name in ['frozen-inputs.sha256','author-submission.sha256']:
 p=subprocess.run(['sha256sum','-c',str(E/name)],cwd=R,capture_output=True)
 (O/(name+'.stdout')).write_bytes(p.stdout); (O/(name+'.stderr')).write_bytes(p.stderr)
 log.append(f'sha256sum -c {E/name}; cwd={R}; exit={p.returncode}; OK={p.stdout.count(b": OK") }')
protected=json.loads((E/'protected-checkpoint.json').read_text()); bad=[]
for name,h in protected.items():
 p=R/name
 actual=hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else 'MISSING'
 if actual!=h: bad.append((name,h,actual))
log.append(f'Protected checkpoint: {len(protected)} checked; mismatches={bad}')
for category,src,dst in [('frozen-checkout',E/'frozen',Path('/tmp/hud-fidelity-run1-rS7MUp/checkout')),('archived-author',E/'author',Path('/tmp/hud-fidelity-run1-rS7MUp/author')),('packets',E/'fact-packets',Path('/tmp/hud-fidelity-run1-rS7MUp/packets'))]:
 files=[p for p in src.rglob('*') if p.is_file()]; bad=[]
 for p in files:
  q=dst/p.relative_to(src)
  if not q.exists() or p.read_bytes()!=q.read_bytes(): bad.append(str(p.relative_to(src)))
 log.append(f'{category}: {len(files)} files compared, differences={bad}')
log.append(f'prompt copy identical: {(E/"author-prompt.md").read_bytes()==Path("/tmp/hud-fidelity-run1-rS7MUp/prompt.md").read_bytes()}')
for p in sorted((E/'author/final').iterdir()):
 for stage in ['drafts','revisions/r1','revisions/r2']:
  q=E/'author'/stage/p.name
  if q.exists():
   log.append(f'{p.name} final vs {stage}: identical={p.read_bytes()==q.read_bytes()}')
   (O/(p.name+'.'+stage.replace('/','-')+'.diff')).write_text(''.join(difflib.unified_diff(q.read_text().splitlines(True),p.read_text().splitlines(True),fromfile=str(q),tofile=str(p))))
(O/'integrity.txt').write_text('\n'.join(log)+'\n')
for trace in sorted((E/'trace').glob('*.gz')):
 records=[json.loads(l) for l in gzip.open(trace,'rt')]; calls=[]; summary=[]
 for i,r in enumerate(records,1):
  if r.get('type')=='message':
   m=r['message']; role=m.get('role'); summary.append(f'{i}: {role} {r.get("timestamp","")}')
   for c in m.get('content',[]) if isinstance(m.get('content'),list) else []:
    if c.get('type')=='toolCall': calls.append({'line':i,**c})
   if role=='user': summary.append(json.dumps(m,ensure_ascii=False))
  elif r.get('type')=='tool_execution_start': calls.append({'line':i,**r})
  elif r.get('type') in ['session','model_change','thinking_level_change','agent_start','agent_end','error']: summary.append(f'{i}: '+json.dumps(r,ensure_ascii=False)[:3000])
 (O/(trace.stem+'.calls.json')).write_text(json.dumps(calls,ensure_ascii=False,indent=2))
 (O/(trace.stem+'.summary.txt')).write_text('\n'.join(summary))
 print(trace.name,len(records),len(calls),collections.Counter(r.get('type') for r in records))
print('\n'.join(log))
