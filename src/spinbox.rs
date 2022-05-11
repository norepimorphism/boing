// SPDX-License-Identifier: MPL-2.0

//! [`Spinbox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Spinbox`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_spinbox(
        &self,
        min: u16,
        max: u16,
    ) -> Result<Spinbox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewSpinbox(min.into(), max.into()) -> Spinbox,
        )
    }
}

def_subcontrol!(
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Spinbox,
    handle: uiSpinbox,
    cb_fns: [ on_changed<'a>() ],
);

impl<'a> Spinbox<'a> {
    bind_ty_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: value() -> i32 },
        libui: { fn: uiSpinboxValue() },
    );

    bind_set_ty_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_value(value: u16) },
        libui: { fn: uiSpinboxSetValue() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this spinbox changes.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Spinbox<'a>,
            handle: uiSpinbox,
            fn: on_changed(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiSpinboxOnChanged(),
            cb: { sig: () -> () },
        },
    );
}
