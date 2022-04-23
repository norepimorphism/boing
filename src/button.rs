// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    pub fn create_button(&mut self, text: impl Into<Vec<u8>>) -> Result<Button, crate::Error> {
        call_libui_new_fn!(Button, uiNewButton, make_cstring!(text).as_ptr())
    }
}

def_subcontrol_with_ptr_ty!(Button, uiButton);
