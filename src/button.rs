// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Button`].

use crate::prelude::*;

impl ui!() {
    pub fn create_button(&self, text: impl AsRef<str>) -> Result<&mut Button, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            alloc: alloc_button,
            fn: uiNewButton(text.as_ptr()) -> Button,
        )
    }
}

def_subcontrol!(
    ty: Button,
    handle: uiButton,
    cb_fns: [
        on_clicked(),
    ],
);

impl Button {
    bind_text_fn!(
        docs: "The text displayed within this button.",
        text,
        raw_text,
        text_ptr,
        uiButtonText,
    );

    bind_set_text_fn!(
        docs: "Sets the text displayed within this button.",
        set_text,
        text,
        uiButtonSetText,
    );

    bind_callback_fn!(
        docs: "Sets a callback for when this button is clicked.",
        self: {
            ty: Button,
            handle: uiButton,
            fn: on_clicked(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiButtonOnClicked(),
            cb: {
                sig: () -> (),
            },
        },
    );
}
