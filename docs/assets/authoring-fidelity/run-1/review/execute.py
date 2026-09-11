from pathlib import Path
import subprocess,os,shlex,json,re
O=Path('/tmp/hud-fidelity-run1-rS7MUp/review'); E=Path('/home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1'); cwd='/tmp/hud-fidelity-run1-rS7MUp/checkout'
env=os.environ.copy(); env.update(CARGO_NET_OFFLINE='true',CARGO_TARGET_DIR=str(O/'target'),TMPDIR=str(O))
D=O/'independent'; D.mkdir(exist_ok=True); records=[]
for p in sorted((E/'author/evidence/final').glob('*.command')):
 args=shlex.split(p.read_text())
 if args[0]!='cargo': continue
 args[args.index('--')+1]=str(E/'author/final'/Path(args[args.index('--')+1]).name)
 args.insert(2,'--locked')
 prefix=D/p.stem
 command=f'cd {cwd} && CARGO_NET_OFFLINE=true CARGO_TARGET_DIR={O}/target TMPDIR={O} '+shlex.join(args)
 prefix.with_suffix(prefix.suffix+'.command').write_text(command+'\n')
 run=subprocess.run(args,cwd=cwd,env=env,capture_output=True)
 for suffix,value in [('stdout',run.stdout),('stderr',run.stderr),('exit',f'{run.returncode}\n'.encode())]: Path(str(prefix)+'.'+suffix).write_bytes(value)
 original=p.with_suffix('.stdout').read_bytes()
 records.append({'name':p.stem,'exit':run.returncode,'stderr_bytes':len(run.stderr),'stdout_identical_to_author':run.stdout==original,'footer':run.stdout.decode().splitlines()[-1] if run.stdout else ''})
(O/'independent-results.json').write_text(json.dumps(records,indent=2)+'\n')
print(f'{len(records)} independent invocations; failures={sum(r["exit"]!=0 for r in records)}; stdout differences={sum(not r["stdout_identical_to_author"] for r in records)}')
