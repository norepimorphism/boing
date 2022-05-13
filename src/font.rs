// SPDX-License-Identifier: MPL-2.0

//!

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Picker`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_font_picker() -> Result<Picker, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewFontButton() -> Picker,
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
    ty: Picker,
    handle: uiFontButton,
);

pub struct Font {

}
