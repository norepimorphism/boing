// SPDX-License-Identifier: MPL-2.0

//! A clickable button containing customizable text.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Button`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_button(&self, text: impl AsRef<str>) -> Result<Button, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            fn: uiNewButton(text.as_ptr()) -> Button,
        )
    }
}

def_subcontrol!(
    docs: r#"
        A clickable button containing customizable text.

        # Examples

        ```no_run
        # fn main() -> Result<(), boing::Error> {
        use boing::{Ui, Window};

        let ui: Ui;
        # let ui = Ui::new()?;
        let window: Window;
        # let window = ui.create_window("", 0, 0, false, false)?;

        let mut button = ui.create_button("Click Me!")?;
        window.set_child(&mut button);

        button.on_clicked(|button| {
            button.disable();
            let _ = button.set_text("Oops, You Can't Click Me Anymore!");
        });
        #
        # Ok(())
        # }
        ```
    "#,
    ty: Button,
    handle: uiButton,
    cb_fns: [ on_clicked<'a>() ],
);

impl<'ui> Button<'ui> {
    bind_text_fn!(
        docs: "
            The text displayed within this button.

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
        libui: { fn: uiButtonText() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the text displayed within this button.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) },
        libui: { fn: uiButtonSetText() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this button is clicked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Button<'ui>,
            handle: uiButton,
            fn: on_clicked(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiButtonOnClicked(),
            cb: { sig: () -> () },
        },
    );
}
