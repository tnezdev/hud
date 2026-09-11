from pathlib import Path
import json,re
E=Path('/home/tnez/Work/tnezdev/hud/docs/assets/authoring-fidelity/run-1'); O=Path('/tmp/hud-fidelity-run1-rS7MUp/review'); A=E/'author'; old=Path('/tmp/hud-fidelity-run1-rS7MUp/author')
lines=[]
for packet,name in [('a','a.compare.json'),('b','b.sequence.json'),('c','c.compare.json'),('d','d.sequence.json'),('e','e.prose.md')]:
 sections=[f'FINAL SOURCE: {old/"final"/name}\n{(A/"final"/name).read_text()}\n']
 for command in sorted((A/'evidence/final').glob(f'{packet}.*.command')):
  sections.append(f'COMMAND FILE: {old/command.relative_to(A)}\n{command.read_text()}')
  for suffix in ['exit','stderr','stdout']:
   p=command.with_suffix('.'+suffix); text=p.read_text()
   sections.append(f'{suffix.upper()} ({len(p.read_bytes())} bytes):\n{text}'+ ('\n' if text else '(empty)\n'))
 lines.append(f'Bundle {packet}: exact collation={chr(10).join(sections)==(A/"evidence/inspection"/f"{packet}.txt").read_text()}')
# Body line positions are taken from actual footers, not requested offsets.
for packet in 'abcd':
 for size in ['72x35','80x24']:
  rows=[]; seen={}; conflicts=[]
  files=sorted((A/'evidence/final').glob(f'{packet}.{size}.*.stdout'),key=lambda p:int(p.stem.split('.')[-1]))
  for p in files:
   text=p.read_text().splitlines(); start,total=map(int,re.search(r'line (\d+)/(\d+)',text[-1]).groups()); body=text[6:-1]
   # At c.80x24.25 the final body cell is intentionally blank: deferred
   # criterion heading, compare.rs 488-490. It is shown at next request 35.
   if packet=='c' and size=='80x24' and p.stem.endswith('.25'): body=body[:-1]
   end=min(start+len(body)-1,total)
   for index,row in zip(range(start,end+1),body):
    # Sequence reidentifies ownership in the first line of a continuation.
    normalized=re.sub(r'^  \d\d [│…]', '     │',row)
    if index in seen and seen[index]!=normalized: conflicts.append(index)
    seen[index]=normalized
   rows.append(f'{p.stem.split(".")[-1]} -> effective {start-1}, body {start}-{end}/{total}')
  lines.append(f'{packet.upper()} {size}: '+ '; '.join(rows))
  lines.append(f'  union={len(seen)}/{total}; missing={sorted(set(range(1,total+1))-set(seen))}; conflicting overlapping rows={conflicts}')
  (O/f'body-{packet}-{size}.txt').write_text('\n'.join(f'{i:02} {seen.get(i,"[NOT COVERED]")}' for i in range(1,total+1))+'\n')
(O/'coverage.txt').write_text('\n'.join(lines)+'\n')
print('\n'.join(lines))
