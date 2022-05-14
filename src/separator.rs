// SPDX-License-Identifier: MPL-2.0

//! [`Separator`].

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`Separator`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_horizontal_separator<'ui>(&'ui self) -> Result<&'ui mut Separator, crate::Error> {
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
    pub fn create_vertical_separator<'ui>(&'ui self) -> Result<&'ui mut Separator, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalSeparator() -> Separator,
        )
    }
}

def_subcontrol!(
    docs: "
        A visual linear separator.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Separator,
    handle: uiSeparator,
);
