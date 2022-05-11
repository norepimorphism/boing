// SPDX-License-Identifier: MPL-2.0

//! [`Slider`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Slider`].
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
    docs: "",
    ty: Slider,
    handle: uiSlider,
    cb_fns: [
        on_changed<'a>(),
    ],
);

impl<'a> Slider<'a> {
    bind_bool_fn!(
        docs: "Determines if this slider has a tooltip.",
        self: { fn: has_tooltip() },
        libui: { fn: uiSliderHasToolTip() },
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this slider has a tooltip.",
        self: { fn: set_has_tooltip() },
        libui: { fn: uiSliderSetHasToolTip() },
    );

    bind_callback_fn!(
        docs: "Sets a callback for when this slider changes.",
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
}
