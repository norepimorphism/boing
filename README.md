# boing

[![crates.io](https://img.shields.io/crates/v/boing)](https://crates.io/crates/boing)
[![docs](https://docs.rs/boing/badge.svg)](https://docs.rs/boing)
[![MPL 2.0 licensed](https://img.shields.io/badge/license-MPL2-green)](./LICENSE)

A safe, lightweight wrapper over [*libui-ng-sys*](https://crates.io/crates/libui-ng-sys).

## Background

[*libui*](https://github.com/andlabs/libui) is a C library that provides a neutral interface to native GUI technologies (e.g., windows, widgets) on major OSes. [*libui-ng*](https://github.com/libui-ng/libui-ng) is the "next generation" of *libui*, developed and maintained separately. *libui-ng-sys* provides Rust bindings to *libui-ng*, and *boing* is a safe yet fairly unopinionated layer on top of *libui-ng-sys*.

Currently, *boing* only links with *libui-ng*&mdash;not the original *libui*. However, *libui-ng-sys* may be updated in the future to support a *libui* feature flag, in which case it should be trivial to update *boing* as well.

## Features

### Lightweight

*TODO*

## Pitfalls

*TODO*

## Terminology

In the context that *boing* uses them, the terms "widget" and "control" are *not* interchangeable. A **widget** is an interactive visual element, while **controls** are a specific subset of widgets that implement `DerefMut<Target = boing::Control>`. In particular, all widgets are controls except for `Menu` and `MenuItem`.

## Design

See [DESIGN.md](./DESIGN.md) for an explanation of how *boing* was designed.

## Project Progress

| Feature       | Docs? | Impl? |
| ------------- | ----- | ----- |
| `Area`        | ☐    | ☐    |
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
| `UniBox`      | ☐    | ☑    |
| `Window`      | ☐    | ☑    |
