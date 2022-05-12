// SPDX-License-Identifier: MPL-2.0

//! [`Separator`].

use crate::prelude::*;

def_subcontrol!(
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Separator,
    handle: uiSeparator,
);

impl Ui {
    /// Creates a new horizontal [`Separator`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_horizontal_separator(&self) -> Result<Separator, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewHorizontalSeparator() -> Separator,
        )
    }

    /// Creates a new vertical [`Separator`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_vertical_separator(&self) -> Result<Separator, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalSeparator() -> Separator,
        )
    }
}
