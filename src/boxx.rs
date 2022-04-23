// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    pub fn create_horizontal_box(&mut self) -> Result<Boxx, crate::Error> {
        call_libui_new_fn!(Boxx, uiNewHorizontalBox)
    }

    pub fn create_vertical_box(&mut self) -> Result<Boxx, crate::Error> {
        call_libui_new_fn!(Boxx, uiNewVerticalBox)
    }
}

def_subcontrol_with_ptr_ty!(Boxx, uiBox);
