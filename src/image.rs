// SPDX-License-Identifier: MPL-2.0

//! [`Image`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Image`].
    pub fn create_image<'a>(
        &'a self,
        width: f64,
        height: f64,
    ) -> Result<&'a mut Image<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_image,
            fn: uiNewImage(width, height) -> Image,
        )
    }
}

def_subcontrol!(ty: Image, handle: uiImage,);

#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}

impl Image<'_> {
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
