// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Label`].
    pub fn create_label(&mut self, text: impl AsRef<str>) -> Result<Label, crate::Error> {
        let text = make_cstring!(text.as_ref());
        call_libui_new_fn!(self, true, Label, uiNewLabel, text.as_ptr())
    }
}

def_subcontrol!(Label, uiLabel);

impl Label {
    bind_text_fn!(
        "The text displayed by this label.",
        text,
        raw_text,
        text_ptr,
        uiLabelText,
    );

    bind_set_text_fn!(
        "Sets the text displayed by this label.",
        set_text,
        text,
        uiLabelSetText,
    );
}
