# iced-wgpu-nastyfork

Iced 0.14's GPU renderer ported to **wgpu 30**, allowing
[iced_egui](https://github.com/DaveWK/iced_egui) to share egui 0.36's GPU device
and queue. Iced widgets/runtime and plotters-iced2 stay on 0.14.

This is a standalone extraction of `iced_wgpu`, not a fork of the entire Iced
workspace. See [UPSTREAM.md](UPSTREAM.md) for release provenance. The Cargo
package remains `iced_wgpu` 0.14.0 and is not published to crates.io.
Rust 1.95 or newer is required by this dependency stack.

## Usage

Both forks must be patched at the **application workspace root**. Cargo ignores
patches in dependencies, including this repository's standalone cryoglyph patch.

```toml
[patch.crates-io]
iced_wgpu = { git = "https://github.com/DaveWK/iced-wgpu-nastyfork", branch = "main" }
cryoglyph = { git = "https://github.com/DaveWK/cryoglyph-nastyfork", rev = "078fab8b8425c5da41465d4aca7fb78c5af3b7fe" }
```

Replace the renderer's moving branch with an immutable `rev` for reproducible
applications. The `iced_egui` migration branch pins both exact revisions.
Use `cargo tree -i wgpu@30.0.1` to inspect the shared renderer dependency.
Do not pass GPU objects from wgpu 27 or 29 into this renderer.

For wasm with eframe 0.36 use `default-features = false` and
`features = ["wgpu_no_default_features"]`. The plain `wgpu` feature enables
`fragile-send-sync-non-atomic-wasm`, incompatible with Iced 0.14's window bounds.

## Port scope

Updates staging belts and mapped uploads, pipeline layouts, optional vertex
buffer layouts, render passes, samplers, adapter selection, owned native display
handles, surface acquisition/configuration and queue presentation. Browser
instances omit native display handles. The headless shared-device path keeps
render targets on the GPU; no CPU-copy bridge or cross-version handle casts.

## Validation

```sh
cargo fmt --all --check
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo test --all-features --test wgpu30 -- --ignored --nocapture
cargo check --target wasm32-unknown-unknown --features geometry,webgl
```

The Vulkan pixel-readback regression covers quads, bundled-font text, geometry,
odd-width raster atlas uploads, SVGs, resize/DPI changes, and MSAA. The companion
glyph fork tests upload-buffer growth and reuse. Existing upstream lint categories
are retained/allowed where needed to avoid unrelated API and allocation changes.

Native window presentation is compile-checked, but on-screen behavior, browser
rendering, Metal and DX12 remain unverified. No hosted Ubuntu CI is configured;
self-hosted Fedora/RHEL runner registration is deferred. Run the bridge's GPU
tests with `--test-threads=1`; concurrent software Vulkan tests exhibited a process
crash during local validation.

## Contributing

Keep the Iced 0.14 public API intact except for its wgpu types. Include a regression
test when changing rendering or resource lifetimes. Prefer small commits that
can be compared against the imported upstream baseline.

## License

[MIT](LICENSE), preserving the upstream Iced copyright notice.
