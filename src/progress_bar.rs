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
    bind_ty_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: value() -> i32 },
        libui: { fn: uiProgressBarValue() },
    );

    bind_set_ty_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_value(value: i32) },
        libui: { fn: uiProgressBarSetValue() },
    );
}
