# Upstream provenance

This standalone repository imports the released `iced_wgpu 0.14.0` sources
from crates.io. The initial commit preserves the release source snapshot;
subsequent commits contain the wgpu 30 port. It is not a complete clone of
the upstream Git history.

- Upstream: https://github.com/iced-rs/iced
- Recorded release commit: `3997291f318a8bc06fa522f5579836fb3feb94df`
- Source: https://github.com/iced-rs/iced/tree/3997291f318a8bc06fa522f5579836fb3feb94df/wgpu
- Original Cargo manifest: `Cargo.toml.orig`
- Release provenance: `.cargo_vcs_info.json`
- License: MIT; original notices are retained.

Cargo package names and versions remain unchanged so application-level
`[patch.crates-io]` can replace the renderer dependencies. `publish = false`
prevents accidental publication under the upstream package name. Use pinned
Git revisions; no crates.io release of this fork is provided.
