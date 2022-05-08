// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Slider`].

use crate::prelude::*;

impl ui!() {
    /// Creates a new [`Slider`].
    pub fn create_slider(&self, min: u16, max: u16) -> Result<&mut Slider, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            alloc: alloc_slider,
            fn: uiNewSlider(min.into(), max.into()) -> Slider,
        )
    }
}

def_subcontrol!(
    ty: Slider,
    handle: uiSlider,
    cb_fns: [
        on_changed(),
    ],
);

impl Slider {
    bind_bool_fn!(
        docs: "Determines if this slider has a tooltip.",
        has_tooltip,
        uiSliderHasToolTip,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this slider has a tooltip.",
        set_has_tooltip,
        uiSliderSetHasToolTip,
    );

    bind_callback_fn!(
        docs: "Sets a callback for when this slider changes.",
        self: {
            ty: Slider,
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
