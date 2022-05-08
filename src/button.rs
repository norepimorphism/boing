// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Button`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    pub fn create_button<'a>(&'a self, text: impl AsRef<str>) -> Result<&'a mut Button<'ui>, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
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

impl<'ui> Button<'ui> {
    bind_text_fn!(
        docs: "The text displayed within this button.",
        self: {
            fn: text,
            raw_fn: raw_text,
            as_ptr_fn: text_ptr,
        },
        libui: {
            fn: uiButtonText(),
        },
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
            ty: Button<'ui>,
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
