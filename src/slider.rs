// SPDX-License-Identifier: MPL-2.0

//! [`Slider`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Slider`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_slider(
        &self,
        min: u16,
        max: u16,
    ) -> Result<Slider, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewSlider(min.into(), max.into()) -> Slider,
        )
    }
}

def_subcontrol!(
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Slider,
    handle: uiSlider,
    cb_fns: [
        on_changed<'a>(),
    ],
);

impl<'a> Slider<'a> {
    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: value() -> i32 },
        libui: { fn: uiSliderValue() },
    );

    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_value(value: u16) },
        libui: { fn: uiSliderSetValue() },
    );

    bind_bool_fn!(
        docs: "
            Determines if this slider has a tooltip.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: has_tooltip() -> bool },
        libui: { fn: uiSliderHasToolTip() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this slider has a tooltip.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_has_tooltip(value: bool) },
        libui: { fn: uiSliderSetHasToolTip() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this slider changes.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Slider<'a>,
            handle: uiSlider,
            fn: on_changed(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiSliderOnChanged(),
            cb: {
                sig: () -> (),
            },
        },
    );

    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_range(min: u16, max: u16) },
        libui: { fn: uiSliderSetRange() },
    );
}
