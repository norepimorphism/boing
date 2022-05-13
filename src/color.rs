// SPDX-License-Identifier: MPL-2.0

//!

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
    pub fn create_color_picker(&self) -> Result<Picker, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewColorButton() -> Picker,
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
    handle: uiColorButton,
    cb_fns: [
        on_selected<'a>(),
    ],
);

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl<'a> Picker<'a> {
    pub fn selected_color(&self) -> Color {
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
            Sets a callback for when a new color is selected.

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
