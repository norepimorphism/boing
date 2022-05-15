// SPDX-License-Identifier: MPL-2.0

//! A toggleable button with adjacent customizable text.

use crate::prelude::*;

impl Ui {
    pub fn create_checkbox<'ui>(
        &'ui self,
        text: impl AsRef<str>,
    ) -> Result<&'ui mut Checkbox, crate::Error> {
        // SAFETY: `uiNewCheckbox` `strdup`s `text`, so it's OK to drop at the end of scope.
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            fn: uiNewCheckbox(text.as_ptr()) -> Checkbox,
        )
    }
}

def_subcontrol!(
    docs: "
        A toggleable button with adjacent customizable text.

        # Examples

        ```no_run
        // TODO
        ```

        # Screenshots

        ## Windows

        ## macOS

        ## Linux
    ",
    ty: Checkbox,
    handle: uiCheckbox,
    cb_fns: [ on_toggled() ],
);

impl<'ui> Checkbox<'ui> {
    bind_text_fn!(
        docs: "
            The text displayed adjacent to this checkbox.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: text(),
            raw_fn: raw_text(),
            as_ptr_fn: text_ptr(),
        },
        libui: { fn: uiCheckboxText() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the text displayed adjacent to this checkbox.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) -> () },
        libui: { fn: uiCheckboxSetText() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this checkbox is toggled.

            Checkboxes toggle when clicked, but otherwise, this callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Checkbox<'ui>,
            handle: uiCheckbox,
            fn: on_toggled(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiCheckboxOnToggled(),
            cb: { sig: () -> () },
        },
    );

    bind_bool_fn!(
        docs: "
            Determines if this checkbox is checked.

            Checkboxes are unchecked by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: checked() -> bool },
        libui: { fn: uiCheckboxChecked() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this checkbox is checked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_checked(value: bool) },
        libui: { fn: uiCheckboxSetChecked() },
    );
}
