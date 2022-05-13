# boing

[![crates.io](https://img.shields.io/crates/v/boing)](https://crates.io/crates/boing)
[![docs](https://docs.rs/boing/badge.svg)](https://docs.rs/boing)
[![MPL 2.0 licensed](https://img.shields.io/badge/license-MPL2-green)](./LICENSE)

A safe, lightweight wrapper over [*libui-ng-sys*](https://crates.io/crates/libui-ng-sys).

## Background

[*libui*](https://github.com/andlabs/libui) is a C library that provides a neutral interface to native GUI technologies (e.g., windows, widgets) on major OSes. [*libui-ng*](https://github.com/libui-ng/libui-ng) is the "next generation" of *libui*, developed and maintained separately. *libui-ng-sys* provides Rust bindings to *libui-ng*, and *boing* is a safe yet fairly unopinionated layer on top of *libui-ng-sys*.

Currently, *boing* only links with *libui-ng*&mdash;not the original *libui*. However, *libui-ng-sys* may be updated in the future to support a *libui* feature flag, in which case it should be trivial to update *boing* as well.

## Terminology

In the context that *boing* uses them, the terms "widget" and "control" are *not* interchangeable. A **widget** is an interactive visual element, while **controls** are a specific subset of widgets that implement `DerefMut<Target = boing::Control>`. In particular, all widgets are controls except for `Menu` and `MenuItem`.

## Design

See [DESIGN.md](./DESIGN.md) for an explanation of how *boing* was designed.

## Limitations

*boing* currently boxes callbacks passed to methods such as `Window::on_closing` and `Button::on_clicked`. This incurs a small performance and memory cost. However, this is an intentional choice for the purpose of convenience. For example, if callbacks were instead borrowed rather than owned by a transfer of ownership, the following code would fail to compile:

```rust
use boing::{Button, Error, Ui, Window};

fn main() -> Result<(), Error> {
    let ui: Ui;
    let window: Window;

    let mut button = create_button(&ui, "Hello World!".into())?;
    window.set_child(&mut button);

    window.show();
    ui.run();

    Ok(())
}

fn create_button<'cb>(ui: &Ui, text: &'cb String) -> Result<Button<'cb>, Error> {
    let button = ui.create_button("Press Me!")?;

    button.on_clicked(
        // This closure is dropped at the end of scope, so its lifetime ends before that of
        // `button`. It is not coerced to `fn()` because it captures `text`.
        &|| println!("{}", text),
    );

    button
}
```

In this case, the closure passed to `Button::on_clicked` would need to be routed through `create_button` as an argument. Such a hindrance was deemed untenable, hence the current callback design.

## Project Progress

| Feature       | Docs? | Impl? | *libui-ng* Type |
| ------------- | ----- | ----- | --------------- |
| `Area`        | ☐    | ☐    | `uiArea`
| `Axis`        | ☐    | ☑    | `uiBox`
| `Checkbox`    | ☐    | ☑    | `uiCheckbox`
| `ColorPicker` | ☐    | ☐    | `uiColorButton`
| `Combobox`    | ☐    | ☐    | `uiCombobox`
| `FontPicker`  | ☐    | ☐    | `uiFontButton`
| `Form`        | ☐    | ☐    | `uiForm`
| `FormEntry`   | ☐    | ☐    | `uiEntry`
| `Grid`        | ☐    | ☐    | `uiGrid`
| `Group`       | ☐    | ☑    | `uiGroup`
| `Image`       | ☐    | ☑    | `uiImage`
| `Label`       | ☐    | ☑    | `uiLabel`
| `Menu`        | ☐    | ☑    | `uiMenu`
| `MenuItem`    | ☐    | ☑    | `uiMenuItem`
| `Path`        | ☐    | ☐    | `uiDrawPath`
| `ProgressBar` | ☐    | ☑    | `uiProgressBar`
| `Pushbutton`  | ☐    | ☑    | `uiButton`
| `RadioButtons`| ☐    | ☑    | `uiRadioButtons`
| `Separator`   | ☐    | ☑    | `uiSeparator`
| `Slider`      | ☐    | ☑    | `uiSlider`
| `Spinbox`     | ☐    | ☑    | `uiSpinbox`
| `Tab`         | ☐    | ☑    | `uiTab`
| `Table`       | ☐    | ☐    | `uiTable`
| `TimePicker`  | ☐    | ☐    | `uiDateTimePicker`
| `Window`      | ☐    | ☑    | `uiWindow`
