// SPDX-License-Identifier: MPL-2.0

use std::ptr::addr_of_mut;

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Picker`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_color_picker<'ui>(&'ui self) -> Result<&'ui mut Picker, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewColorButton() -> Picker,
        )
    }
}

def_subcontrol!(
    docs: "
        A pushbutton that opens a color picker dialog.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Picker,
    handle: uiColorButton,
    cb_fns: [ on_selected<'a>() ],
);

/// An RGBA color.
#[derive(Clone, Copy, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl<'a> Picker<'a> {
    /// The color currently selected by this picker.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn selected_color(&self) -> Color {
        // TODO: Can we use `MaybeUninit` here, or is that too risky in case of future breakage? It
        // is UB to read from `MaybeUninit` data, after all.
        let mut color = Color::default();
        unsafe {
            uiColorButtonColor(
                self.as_ptr(),
                addr_of_mut!(color.red),
                addr_of_mut!(color.green),
                addr_of_mut!(color.blue),
                addr_of_mut!(color.alpha),
            )
        };

        color
    }

    /// Selects a new color.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn select_color(&self, color: Color) {
        unsafe {
            uiColorButtonSetColor(
                self.as_ptr(),
                color.alpha,
                color.green,
                color.blue,
                color.alpha,
            );
        }
    }

    bind_callback_fn!(
        docs: "
            Sets a callback for when the selected color changes.

            This callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Picker<'a>,
            handle: uiColorButton,
            fn: on_selected(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiColorButtonOnChanged(),
            cb: {
                sig: () -> (),
            },
        },
    );
}
