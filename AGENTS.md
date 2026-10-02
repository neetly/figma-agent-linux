# Instructions for AI Agents

- Read [README.md](README.md) for project behavior and
  [CONTRIBUTING.md](CONTRIBUTING.md) for the LLM contribution policy.
- Work under human direction. Do not independently initiate and submit
  contributions. Make your changes and reasoning clear enough for the human
  contributor to review, understand, and take responsibility for them.
- Include a `Co-authored-by:` trailer identifying the assisting model or tool
  in every commit containing AI-assisted work, including amended commits.
- Keep changes focused on the requested task and follow existing conventions.
  Update documentation when changing user-visible behavior or configuration.
- Keep README.md focused on user-facing behavior and configuration; do not add
  implementation details.
- For Rust changes, use the toolchain in `rust-toolchain.toml` and run the CI
  checks: `cargo +nightly fmt --check`, `cargo clippy -- --deny warnings`, and
  `cargo test`. For documentation-only changes, check the diff and links;
  Rust checks are unnecessary.
- Report what changed, what validation actually ran, and any remaining
  limitations. Do not claim that checks passed if they were not run.
