# 🚢 Releasing tasmiyah-cli

This document is for maintainers. End users don't need to read it.

Releases are fully automated by [`.github/workflows/release.yml`](../.github/workflows/release.yml).
You cut a release by **pushing a version tag**; everything else happens in CI.

---

## TL;DR

```bash
# 1. Bump the version in Cargo.toml (e.g. 0.1.0 -> 0.2.0)
$EDITOR Cargo.toml

# 2. Refresh Cargo.lock so the bump is recorded
cargo build

# 3. Commit
git add Cargo.toml Cargo.lock
git commit -m "chore: release v0.2.0"

# 4. Tag and push
git tag -a v0.2.0 -m "v0.2.0"
git push origin main --follow-tags
```

Once the tag lands, the **Release** workflow:

1. Creates a draft GitHub Release named `v0.2.0`.
2. Builds prebuilt binaries on a matrix of 5 targets (see below).
3. Uploads `.tar.gz` / `.zip` archives **and** matching `.sha256` checksums to the release.
4. Publishes the crate to **crates.io** (`cargo publish`).
5. Bumps the formula in the **`bhantsi/homebrew-tap`** repository so `brew install bhantsi/tap/tasmiyah-cli` picks up the new version.
6. Flips the GitHub Release from draft to public.

You can then go to the [Releases page](https://github.com/bhantsi/tasmiyah-cli/releases) and confirm everything looks right.

---

## One-time setup

Before the very first release, configure two repository secrets under
*Settings → Secrets and variables → Actions*:

| Secret | What it is | How to get it |
|---|---|---|
| `CARGO_REGISTRY_TOKEN` | API token for `cargo publish`. | Generate at <https://crates.io/me> → *API Tokens*. Give it the `publish-update` scope (and `publish-new` for the very first release). |
| `HOMEBREW_TAP_TOKEN` | Fine-grained Personal Access Token with `contents: write` on the `bhantsi/homebrew-tap` repo. | <https://github.com/settings/personal-access-tokens/new> → select the tap repo → Repository permissions → *Contents: Read and write*. |

The `bump-homebrew` job auto-skips if `HOMEBREW_TAP_TOKEN` isn't set, so
you can release to crates.io + GitHub Releases without the tap step.

### Bootstrapping the Homebrew tap

The tap is a separate Git repo, conventionally named `homebrew-<tapname>`:

```bash
# Create an empty repo at https://github.com/bhantsi/homebrew-tap
git clone https://github.com/bhantsi/homebrew-tap
cd homebrew-tap
mkdir -p Formula
echo "# bhantsi/tap" > README.md
git add .
git commit -m "init"
git push
```

The release workflow will commit `Formula/tasmiyah-cli.rb` to this repo on
the next tag push. The formula is rendered from
[`docs/homebrew/tasmiyah-cli.rb.tmpl`](homebrew/tasmiyah-cli.rb.tmpl); edit
that template if you ever change install behavior.

Users install with:

```bash
brew install bhantsi/tap/tasmiyah-cli
```

---

## What gets built

| Target triple | Runner | Archive |
|---|---|---|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `.tar.gz` |
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | `.tar.gz` (static) |
| `aarch64-unknown-linux-musl` | `ubuntu-latest` | `.tar.gz` (static) |
| `aarch64-apple-darwin` | `macos-latest` | `.tar.gz` |
| `x86_64-pc-windows-msvc` | `windows-latest` | `.zip` |

Each archive contains the compiled `tasmiyah` binary and is accompanied by a `.sha256` file. Intel Macs run the `aarch64-apple-darwin` build under Rosetta 2.

---

## Pre-release checklist

Before tagging:

- [ ] `cargo fmt --all -- --check` is clean.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` is clean.
- [ ] `cargo test` passes locally.
- [ ] CI is green on `main` (the same checks are enforced there).
- [ ] `Cargo.toml` `version` is bumped (and committed) following [semver](https://semver.org/).
- [ ] User-visible changes are reflected in the README / user guide if needed.

---

## Choosing a version

We follow [Semantic Versioning](https://semver.org/):

- **Patch** (`0.1.0 → 0.1.1`): bug fix, no behavior change.
- **Minor** (`0.1.0 → 0.2.0`): new flag, new phrase, additive feature.
- **Major** (`0.x.y → 1.0.0`): breaking CLI change or first stable release.

While we're pre-1.0, any breaking change can ship in a minor bump, but try to avoid them.

---

## Testing the workflow without cutting a real release

The release workflow accepts a `workflow_dispatch` trigger with a `tag` input.
Pre-create an annotated test tag (e.g. `v0.0.0-test1`), push it, then dispatch the
workflow manually against that tag from the Actions tab. Delete the test release
and tag afterwards:

```bash
git push --delete origin v0.0.0-test1
git tag -d v0.0.0-test1
gh release delete v0.0.0-test1
```

---

## Recovering from a failed release

If one of the matrix builds fails:

1. The release stays in **draft** state — nothing is published.
2. Fix the underlying problem on `main`.
3. Re-run only the failed jobs from the Actions UI, or re-trigger the whole workflow via `workflow_dispatch` with the same tag.

If a release was already published with a bad artifact:

1. Delete the bad asset from the Releases page (or `gh release delete-asset`).
2. Re-run the workflow against the same tag — it will re-upload missing assets without re-creating the release.

If a tag was pushed by mistake:

```bash
git push --delete origin v0.2.0
git tag -d v0.2.0
gh release delete v0.2.0   # also removes the draft, if any
```

Then start over with the correct tag.

---

## After the release

- The crate appears on <https://crates.io/crates/tasmiyah-cli> within a few seconds of `cargo publish` completing.
- The Homebrew formula commit appears at <https://github.com/bhantsi/homebrew-tap/commits/main>.
- Verify a clean install end-to-end:
  ```bash
  cargo install tasmiyah-cli
  brew install bhantsi/tap/tasmiyah-cli
  ```

That's it. 🌙
