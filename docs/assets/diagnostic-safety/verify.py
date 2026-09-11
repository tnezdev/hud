"""One-off evidence check; run from the repo root with before/after binary dirs.

Usage: python3 docs/assets/diagnostic-safety/verify.py BEFORE AFTER OUTPUT.json
Uses direct, captured example processes, never a shell or live terminal. It does
not modify fixtures, frozen evidence, source, or either binary directory.
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile

root = Path.cwd()
before, after, output = map(lambda p: Path(p).resolve(), sys.argv[1:])
results = {"before": str(before), "after": str(after), "valid": [], "rejected": []}


def invoke(directory, name, args):
    run = subprocess.run([str(directory / name), *map(str, args)], capture_output=True)
    return run.returncode, run.stdout, run.stderr


def identical(name, args, archived=None):
    old = invoke(before, name, args)
    new = invoke(after, name, args)
    assert old == new, (name, list(map(str, args)), "changed valid output")
    assert new[0] == 0 and not new[2], (name, "unexpected valid failure")
    if archived is not None:
        assert new[1] == archived.read_bytes(), (name, str(archived), "changed trial capture")
    results["valid"].append({
        "example": name, "args": list(map(str, args)),
        "stdout_sha256": hashlib.sha256(new[1]).hexdigest(),
        "archived_capture": str(archived) if archived else None,
    })


fixtures = [
    ("compare", "compare-suppliers.json"),
    ("compare", "compare-access.json"),
    ("sequence", "sequence-journey.json"),
    ("sequence", "sequence-intake.json"),
    ("sequence", "sequence-observatory.json"),
    ("expressive_story", "coach-story.json"),
    ("expressive_story", "coach-departure.json"),
    ("expressive_story", "coach-arrival.json"),
]
for name, fixture in fixtures:
    for plain in ([[], ["--plain"]] if name == "expressive_story" else [[]]):
        prefix = [root / "examples" / fixture, *plain]
        identical(name, [*prefix, "--check"])
        for width, height in [(64, 12), (72, 35), (80, 24)]:
            for offset in [0, 8, 20, 65535]:
                identical(name, [*prefix, "--preview", width, height, offset])

trial = root / "docs/assets/authoring-fidelity/run-1/author"
for packet, name in [("a", "compare"), ("b", "sequence"), ("c", "compare"), ("d", "sequence")]:
    source = trial / "final" / f"{packet}.{name}.json"
    identical(name, [source, "--check"], trial / "evidence/final" / f"{packet}.check.stdout")
    for capture in sorted((trial / "evidence/final").glob(f"{packet}.*x*.*.stdout")):
        _, size, offset, _ = capture.name.split(".")
        width, height = size.split("x")
        identical(name, [source, "--preview", width, height, offset], capture)

payload = "\x1b[2JPROBE\u009b0m\u202e\n"
expected = r"\u{1b}[2JPROBE\u{9b}0m\u{202e}\n"
with tempfile.TemporaryDirectory(prefix="hud-diagnostic-probe-") as tmp:
    tmp = Path(tmp)
    for name, fixture, pointers, variant in [
        ("compare", "compare-suppliers.json", ["", "/criteria/0", "/alternatives/0", "/alternatives/0/answers/0"], "/alternatives/0/answers/0/state"),
        ("sequence", "sequence-intake.json", ["", "/items/0"], "/relationship"),
        ("expressive_story", "coach-story.json", ["", "/lead", "/route", "/support"], None),
    ]:
        def reject(path, case):
            old = invoke(before, name, [path, "--check"])
            new = invoke(after, name, [path, "--check"])
            assert old[0] == new[0] == 1 and not old[1] and not new[1]
            assert payload.encode() in old[2], (name, case, "not reproducing original defect")
            assert new[2] == old[2].replace(payload.encode(), expected.encode()), (name, case)
            results["rejected"].append({
                "example": name, "case": case, "exit": new[0],
                "before_stderr_hex": old[2].hex(),
                "after_stderr": new[2].decode(),
            })

        for pointer in [*pointers, *([variant] if variant else [])]:
            value = json.loads((root / "examples" / fixture).read_text())
            node = value
            parts = pointer.strip("/").split("/") if pointer else []
            for part in (parts[:-1] if pointer == variant else parts):
                node = node[int(part)] if isinstance(node, list) else node[part]
            if pointer == variant:
                node[parts[-1]] = payload
            else:
                node[payload] = True
            path = tmp / f"{name}.json"
            path.write_text(json.dumps(value, ensure_ascii=True))
            reject(path, f"JSON {pointer or 'root'}")
        reject(tmp / f"missing-{payload}.json", "path-bearing I/O error")

output.write_text(json.dumps(results, indent=2) + "\n")
print(f"PASS: {len(results['valid'])} identical valid invocations, "
      f"{len(results['rejected'])} diagnostic-injection cases rejected and escaped")
