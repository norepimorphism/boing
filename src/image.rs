// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Image`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Image`].
    pub fn create_image(&self, width: f64, height: f64) -> Result<&mut Image, crate::Error> {
        call_libui_new_fn!(self, Image, uiNewImage, width, height)
    }
}

def_subcontrol!(Image, uiImage);

#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}

impl Image {
    pub fn append(&mut self, pixels: &mut [Pixel], width: u16, height: u16, byte_stride: u16) {
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
