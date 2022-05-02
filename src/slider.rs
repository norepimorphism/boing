// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Slider`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Slider`].
    pub fn create_slider(&self, min: u16, max: u16) -> Result<&mut Slider, crate::Error> {
        call_libui_new_fn!(self, Slider, uiNewSlider, min.into(), max.into())
    }
}

def_subcontrol!(Slider, uiSlider);

impl Slider {
    bind_bool_fn!(
        "Determines if this slider has a tooltip.",
        has_tooltip,
        uiSliderHasToolTip,
    );

    bind_set_bool_fn!(
        "Sets whether or not this slider has a tooltip.",
        set_has_tooltip,
        uiSliderSetHasToolTip,
    );

    bind_callback_fn!(
        "Sets a callback for when this slider changes.",
        Slider,
        on_changed,
        uiSliderOnChanged;
        f -> (),
        (),
        : uiSlider,
    );
}
