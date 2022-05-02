// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Spinbox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Spinbox`].
    pub fn create_spinbox(&self, min: u16, max: u16) -> Result<&mut Spinbox, crate::Error> {
        call_libui_new_fn!(self, Spinbox, uiNewSpinbox, min.into(), max.into())
    }
}

def_subcontrol!(Spinbox, uiSpinbox);

impl Spinbox {
    bind_callback_fn!(
        "Sets a callback for when this spinbox changes.",
        Spinbox,
        on_changed,
        uiSpinboxOnChanged;
        f -> (),
        (),
        spinbox: uiSpinbox,
    );
}
