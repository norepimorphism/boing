// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Button`].

use crate::prelude::*;

impl Ui {
    pub fn create_button(&self, text: impl AsRef<str>) -> Result<&mut Button, crate::Error> {
        let text = make_cstring!(text.as_ref());
        call_libui_new_fn!(self, Button, uiNewButton, text.as_ptr())
    }
}

def_subcontrol!(Button, uiButton);

impl Button {
    bind_text_fn!(
        "The text displayed within this button.",
        text,
        raw_text,
        text_ptr,
        uiButtonText,
    );

    bind_set_text_fn!(
        "Sets the text displayed within this button.",
        set_text,
        text,
        uiButtonSetText,
    );

    bind_callback_fn!(
        "Sets a callback for when this button is clicked.",
        on_clicked,
        uiButtonOnClicked;
        f -> (),
        (),
        uiButton,
    );
}
