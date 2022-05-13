// SPDX-License-Identifier: MPL-2.0

//! [`ProgressBar`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`ProgressBar`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_progress_bar(&self) -> Result<ProgressBar, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewProgressBar() -> ProgressBar,
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
    ty: ProgressBar,
    handle: uiProgressBar,
);

impl ProgressBar {
    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: value() -> u16,
            map_out: |_, value| {
                assert_uint!(value);

                value as u16
            },
        },
        libui: { fn: uiProgressBarValue() },
    );

    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_value(value: u16) },
        libui: { fn: uiProgressBarSetValue() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_indefinite() },
        libui: { fn: uiProgressBarSetValue(-1) },
    );
}
