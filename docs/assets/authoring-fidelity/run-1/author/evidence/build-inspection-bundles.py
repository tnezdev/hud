# Evidence collation only, not rendering. Original streams remain untouched.
from pathlib import Path
root = Path('/tmp/hud-fidelity-run1-rS7MUp/author')
out = root / 'evidence' / 'inspection'
out.mkdir(exist_ok=True)
for packet, name in [('a','a.compare.json'), ('b','b.sequence.json'), ('c','c.compare.json'), ('d','d.sequence.json'), ('e','e.prose.md')]:
    source = root / 'final' / name
    sections = [f'FINAL SOURCE: {source}\n{source.read_text()}\n']
    commands = sorted((root/'evidence'/'final').glob(f'{packet}.*.command'))
    for command in commands:
        prefix = command.with_suffix('')
        sections.append(f'COMMAND FILE: {command}\n{command.read_text()}')
        for suffix in ['exit', 'stderr', 'stdout']:
            path = Path(str(prefix) + '.' + suffix)
            text = path.read_text()
            sections.append(f'{suffix.upper()} ({len(path.read_bytes())} bytes):\n{text}' + ('\n' if text else '(empty)\n'))
    (out/f'{packet}.txt').write_text('\n'.join(sections))
    print(f'Wrote inspection/{packet}.txt')
# Audit all contemporaneously saved command records, including failed previews.
sections = []
for stage in ['initial', 'r1', 'middle', 'final']:
    for command in sorted((root/'evidence'/stage).glob('*.command')):
        prefix = command.with_suffix('')
        exit_path = Path(str(prefix)+'.exit')
        stdout_path = Path(str(prefix)+'.stdout')
        stderr_path = Path(str(prefix)+'.stderr')
        sections.append(f'{stage}/{command.name}\n{command.read_text()}exit={exit_path.read_text().strip()}; stdout bytes={stdout_path.stat().st_size}; stderr bytes={stderr_path.stat().st_size}\nstderr verbatim:\n{stderr_path.read_text()}')
(out/'stream-audit.txt').write_text('\n'.join(sections))
print('Wrote inspection/stream-audit.txt')
