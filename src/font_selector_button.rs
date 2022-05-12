// SPDX-License-Identifier: MPL-2.0

//! [`FontSelectorButton`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`FontSelectorButton`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_font_selector_button() -> Result<FontSelectorButton, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewFontButton() -> FontSelectorButton,
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
    ty: FontSelectorButton,
    handle: uiFontButton,
);
