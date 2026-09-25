<p align="center">
  <b>Security-first Rust syntax and macro tooling with a no_std core.</b><br>
  First-party crates, explicit validation, bounded work, and small reviewed releases.
</p>

<div align="center">
  <a href="https://crates.io/crates/synir">Crates.io</a> |
  <a href="https://docs.rs/synir">Docs.rs</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/docs/RELEASE_PLAN.md">Release Plan</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/docs/threat-model.md">Threat Model</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/synir">
    <img src="https://raw.githubusercontent.com/valkyoth/synir/main/.github/images/synir.webp" alt="Synir Rust syntax and macro toolkit">
  </a>
</p>

# synir-xtask

Unpublished, dependency-free Rust host tooling for the Synir foundation.

Run `cargo xtask policy` for source-size, dependency graph, documentation and
roadmap guards. Run `cargo xtask freshness` for an online review of official
Rust and tool releases. No command updates pins or publishes packages.

This tool uses std and external `cargo`, `curl` and `git` executables; it is
outside the portable library graph. Its unit tests cover rejection behavior.
Licensed MIT OR Apache-2.0 under the workspace license files.
