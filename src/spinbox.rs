// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Spinbox`].
    pub fn create_spinbox(&mut self, min: u16, max: u16) -> Result<Spinbox, crate::Error> {
        call_libui_new_fn!(self, true, Spinbox, uiNewSpinbox, min.into(), max.into())
    }
}

def_subcontrol!(Spinbox, uiSpinbox);
