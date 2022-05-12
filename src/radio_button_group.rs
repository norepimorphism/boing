// SPDX-License-Identifier: MPL-2.0

//! [`RadioButtonGroup`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`RadioButtonGroup`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_radio_button_group() -> Result<RadioButtonGroup, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewRadioButtons() -> RadioButtonGroup,
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
    ty: RadioButtonGroup,
    handle: uiRadioButtons,
);
