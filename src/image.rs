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
    pub fn create_image(
        &self,
        width: f64,
        height: f64,
    ) -> Result<Image, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewImage(width, height) -> Image,
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

impl Image {
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn append(&self, pixels: &mut [Pixel], width: u16, height: u16, byte_stride: u16) {
        unsafe {
            uiImageAppend(
                self.as_ptr(),
                pixels.as_mut_ptr().cast(),
                width.into(),
                height.into(),
                byte_stride.into(),
            );
        }
    }
}
