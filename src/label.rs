// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Label`].
    pub fn create_label(&mut self, text: impl Into<Vec<u8>>) -> Result<Label, crate::Error> {
        call_libui_new_fn!(Label, uiNewLabel, make_cstring!(text).as_ptr())
    }
}

def_subcontrol_with_ptr_ty!(Label, uiLabel);
