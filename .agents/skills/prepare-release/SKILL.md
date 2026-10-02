---
name: prepare-release
description: Prepare a new figma-agent-linux release by updating the package version, changelog, and release links, then validating the changes. Use for release preparation or a release version bump in this repository.
---

# Prepare a release

Read the repository's `AGENTS.md`, `README.md`, and `CONTRIBUTING.md` and inspect
the working tree before editing. Preserve unrelated changes.

## Version and baseline

- Inspect `Cargo.toml`, release tags, and `.github/workflows/release.yml` to
  establish the current version and preceding release. Release tags use the
  bare package version, such as `0.4.5`. Do not assume the manifest version has
  already been released or add compatibility for older tag formats.
- Use the version requested by the user. Otherwise choose a patch bump for fixes
  and maintenance, or a minor bump for new features, and explain the choice. Ask
  if the intended version or release baseline cannot be established.
- Update the package version in `Cargo.toml` and the `figma-agent` entry in
  `Cargo.lock`, without updating dependency versions or unrelated lock entries.

## Changelog and release notes

- Review commits and relevant diffs from the preceding release to the intended
  release commit, including pending changes that will ship. Summarize actual
  user-visible fixes and changes; group routine dependency maintenance.
- Add an explicit anchor such as `<a name="0.4.5"></a>` followed by the version
  heading `## 0.4.5` at the top of `CHANGELOG.md`, below its introduction. Keep
  existing entries. The changelog begins at 0.4.4; do not backfill earlier releases.
- Include a full comparison link using the actual preceding and planned tags.
  Do not invent a release date before one has been chosen.
- Keep the GitHub release body linked to `CHANGELOG.md` at the release tag,
  with the bare tag as the anchor. Use `github.repository` and `github.ref_name`
  directly in the workflow URL and fragment. Explicit anchors preserve dots;
  GitHub-generated heading anchors remove them. Keep tags, headings, and
  explicit anchors consistent.
- The tag-push workflow builds x86_64 and aarch64 Linux binaries and publishes
  the GitHub release. Preserve its artifact names because the installer uses
  them. No separate release branch is required by the current workflow.

## Validation and handoff

- Run `git diff --check` and review the full release diff, including new files.
  Check version consistency, changelog scope, comparison tags, and workflow
  link/anchor expansion. Links to the new tag become available after publication.
- For package version changes or Rust edits, use `rust-toolchain.toml` and run
  `cargo +nightly fmt --check`, `cargo clippy -- --deny warnings`, and `cargo test`.
  Report checks that failed or could not run. Documentation-only edits do not
  require Rust checks.
- Report the chosen version, changelog coverage, validations actually run, and
  remaining limitations. Leave a reviewable diff when asked to prepare a release.
- Commit, tag, push, or publish only when the user requests those actions. Before
  an authorized tag push, ensure the tagged commit contains the version bump and
  changelog and required checks have passed. Every AI-assisted commit needs the
  `Co-authored-by:` trailer required by `CONTRIBUTING.md`.
