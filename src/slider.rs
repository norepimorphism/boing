// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Slider`].
    pub fn create_slider(&mut self, min: u16, max: u16) -> Result<Slider, crate::Error> {
        call_libui_new_fn!(self, true, Slider, uiNewSlider, min.into(), max.into())
    }
}

def_subcontrol!(Slider, uiSlider);
