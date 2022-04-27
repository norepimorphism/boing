// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    pub fn create_button(&mut self, text: impl Into<Vec<u8>>) -> Result<Button, crate::Error> {
        let text = make_cstring!(text);
        call_libui_new_fn!(self, Button, uiNewButton, text.as_ptr())
    }
}

def_subcontrol!(Button, uiButton);
