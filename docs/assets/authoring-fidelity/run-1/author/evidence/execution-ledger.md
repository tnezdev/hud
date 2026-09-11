# Execution ledger

All Cargo commands were recorded immediately before execution in `initial/*.command`, `r1/*.command`, `middle/*.command`, and `final/*.command`. Matching `.stdout`, `.stderr`, `.exit` files are original redirected streams/statuses. Each stage's `environment.txt` records the checkout working directory and offline setting. The four shell scripts preserve the exact loops/arguments, including dimensions and offsets. Final copy and hash commands have the same per-command records.

The following is a transcription of the harness shell calls (not a reconstruction of diagnostics). Tool-based authoring used `write` for new artifacts and `edit` for the targeted R1/R2 changes and inspection-log extension. No shell story interpolation was used. All shell calls started in `/tmp/hud-fidelity-run1-rS7MUp/checkout`; Cargo scripts explicitly change to that directory too.

## In execution order

```sh
bash /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-initial.sh > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-initial.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-initial.stderr; status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-initial.exit; printf 'Evidence script exit=%s\n' "$status"
```

```sh
python3 - <<'PY' > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/provenance-lengths.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/provenance-lengths.stderr
import json
from pathlib import Path
root=Path('/tmp/hud-fidelity-run1-rS7MUp/author')
for key in ('a','c'):
    value=json.loads((root/'drafts'/f'{key}.compare.json').read_text())['provenance']
    print(key, len(value), repr(value))
PY
status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/provenance-lengths.exit
```

```sh
mkdir -p /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1; cp /tmp/hud-fidelity-run1-rS7MUp/author/drafts/a.compare.json /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/a.compare.json; cp /tmp/hud-fidelity-run1-rS7MUp/author/drafts/c.compare.json /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/c.compare.json
```

```sh
bash /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-r1.sh > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-r1.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-r1.stderr; status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-r1.exit; printf 'Evidence script exit=%s\n' "$status"
```

```sh
bash /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-middle.sh > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-middle.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-middle.stderr; status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/run-middle.exit; printf 'Evidence script exit=%s\n' "$status"
```

```sh
mkdir -p /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r2; cp /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/a.compare.json /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r2/a.compare.json; cp /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r1/c.compare.json /tmp/hud-fidelity-run1-rS7MUp/author/revisions/r2/c.compare.json
```

```sh
bash /tmp/hud-fidelity-run1-rS7MUp/author/evidence/submit-and-preview.sh > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/submit-and-preview.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/submit-and-preview.stderr; status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/submit-and-preview.exit; printf 'Evidence script exit=%s\n' "$status"
```

```sh
python3 /tmp/hud-fidelity-run1-rS7MUp/author/evidence/build-inspection-bundles.py > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/build-inspection-bundles.stdout 2> /tmp/hud-fidelity-run1-rS7MUp/author/evidence/build-inspection-bundles.stderr; status=$?; printf '%s\n' "$status" > /tmp/hud-fidelity-run1-rS7MUp/author/evidence/build-inspection-bundles.exit; printf 'Collation exit=%s\n' "$status"
```

The inspection bundles were created later by copying saved streams verbatim with labels. They are convenience collations, not original process output. Original evidence is retained in the stage directories. Collector scripts completed with exit 0; individual Cargo statuses, including ten initial failures, are recorded separately. The two preparatory mkdir/cp shell calls produced no tool output; no separate stream files were captured for those calls.
