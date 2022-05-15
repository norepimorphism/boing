// SPDX-License-Identifier: MPL-2.0

//! A clickable button containing customizable text.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Pushbutton`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_pushbutton<'ui>(&'ui self, text: impl AsRef<str>) -> Result<&'ui mut Pushbutton, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            fn: uiNewButton(text.as_ptr()) -> Pushbutton,
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

        let mut button = ui.create_pushbutton("Click Me!")?;
        window.set_child(button);

        button.on_clicked(|button| {
            button.disable();
            let _ = button.set_text("Oops, You Can't Click Me Anymore!");
        });
        #
        # Ok(())
        # }
        ```

        # Screenshots

        ## Windows

        ## macOS

        ## Linux
    "#,
    ty: Pushbutton,
    handle: uiButton,
    cb_fns: [ on_clicked<'a>() ],
);

impl<'ui> Pushbutton<'ui> {
    bind_text_fn!(
        docs: "
            The text displayed within this pushbutton.

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
            Sets the text displayed within this pushbutton.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) -> () },
        libui: { fn: uiButtonSetText() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this pushbutton is clicked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Pushbutton<'ui>,
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
