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
    pub fn create_spinbox<'ui>(&'ui self, min: u16, max: u16) -> Result<&'ui mut Spinbox, crate::Error> {
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
    cb_fns: [ on_changed() ],
);

impl<'ui> Spinbox<'ui> {
    bind_fn!(
        docs: "
            The current value of this spinbox.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: value() -> u16,
            map_out: |_, value| to_u16!(value),
        },
        libui: { fn: uiSpinboxValue() },
    );

    bind_fn!(
        docs: "
            Sets the value of this spinbox.

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

            This callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Spinbox<'ui>,
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
