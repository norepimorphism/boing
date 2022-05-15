// SPDX-License-Identifier: MPL-2.0

//! [`Image`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Image`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_image<'ui>(&'ui self, width: f64, height: f64) -> Result<&'ui mut Image, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewImage(width, height) -> Image,
        )
    }
}

def_subcontrol!(
    docs: "
        An RGBA bitmap.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Image,
    handle: uiImage,
);

#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}

impl Image<'_> {
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn append(&self, pixels: &mut [Pixel], width: u16, height: u16, byte_stride: u16) {
        // SAFETY: [`Pixel`] has a C representation, so it should be castable in this way.
        // SAFETY: `pixels` is dropped at the end of scope, but that's OK as *libui-ng* copies it.
        let pixels = pixels.as_mut_ptr().cast();

        unsafe {
            uiImageAppend(
                self.as_ptr(),
                pixels,
                width.into(),
                height.into(),
                byte_stride.into(),
            );
        }
    }
}
