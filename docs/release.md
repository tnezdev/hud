# Release Process

`hud` releases are cut from `main` with Git tags. Pushing a `v*` tag runs `.github/workflows/release.yml`, builds release binaries, and publishes a GitHub Release with downloadable archives.

## Supported 0.1.x Artifacts

- `hud-x86_64-unknown-linux-gnu.tar.gz`
- `hud-x86_64-apple-darwin.tar.gz`
- `hud-aarch64-apple-darwin.tar.gz`

## Cut A Release

1. Make sure `main` is green in CI.
2. Update `CHANGELOG.md` if the release needs notes beyond generated PR notes.
3. Run `./scripts/check` locally.
4. Create and push a tag:

```sh
git tag v0.1.0
git push origin v0.1.0
```

5. Watch the `Release` workflow.
6. Download the published archive for your platform and smoke-test it:

```sh
tar -xzf hud-aarch64-apple-darwin.tar.gz
./hud-aarch64-apple-darwin/hud --version
./hud-aarch64-apple-darwin/hud --config examples/starter.toml --check-config
```

Do not tag from a feature branch. If the workflow fails, delete the failed GitHub Release, fix `main`, and cut a new patch tag.
