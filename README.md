# boing

[![crates.io](https://img.shields.io/crates/v/boing)](https://crates.io/crates/boing)
[![docs](https://docs.rs/boing/badge.svg)](https://docs.rs/boing)
[![MPL 2.0 licensed](https://img.shields.io/github/license/norepimorphism/boing)](./LICENSE)

A safe, lightweight wrapper over [*libui-ng-sys*](https://crates.io/crates/libui-ng-sys).

## Design

See [DESIGN.md](./DESIGN.md) for an explanation of how *boing* was designed.

## Project Progress

* As a limitation of [Bumpalo](https://crates.io/crates/bumpalo), *boing* currently leaks most resources (e.g., controls, callbacks) allocated by *libui-ng*. While this can be addressed, the solution may require Nightly Rust.

| Feature       | Docs? | Impl? |
| ------------- | ----- | ----- |
| `Area`        | ☐    | ☐    |
| `UniBox`      | ☐    | ☑    |
| `BiBox`       | ☐    | ☐    |
| `Button`      | ☑    | ☑    |
| `Checkbox`    | ☑    | ☑    |
| `Combobox`    | ☐    | ☐    |
| `Grid`        | ☐    | ☐    |
| `Group`       | ☐    | ☑    |
| `Image`       | ☐    | ☑    |
| `Label`       | ☐    | ☑    |
| `Menu`        | ☐    | ☑    |
| `MenuItem`    | ☐    | ☐    |
| `ProgressBar` | ☐    | ☐    |
| `Slider`      | ☐    | ☐    |
| `Spinbox`     | ☐    | ☐    |
| `Tab`         | ☐    | ☑    |
| `Table`       | ☐    | ☐    |
| `Window`      | ☐    | ☑    |
