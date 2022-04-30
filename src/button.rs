// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    pub fn create_button(&mut self, text: impl AsRef<str>) -> Result<Button, crate::Error> {
        let text = make_cstring!(text.as_ref());
        call_libui_new_fn!(self, true, Button, uiNewButton, text.as_ptr())
    }
}

def_subcontrol!(Button, uiButton);

impl Button {
    bind_text_fn!(
        text,
        raw_text,
        text_ptr,
        uiButtonText,
    );

    bind_set_text_fn!(
        set_text,
        text,
        uiButtonSetText,
    );

    bind_callback_fn!(
        on_clicked,
        uiButtonOnClicked;
        (),
        uiButton,
    );
}
