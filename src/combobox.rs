// SPDX-License-Identifier: MPL-2.0

//! [`Combobox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Combobox`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_combobox() -> Result<Combobox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewCombobox() -> Combobox,
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
    ty: Combobox,
    handle: uiCombobox,
);

impl Combobox {}
