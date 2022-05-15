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
    pub fn create_image(&self, width: f64, height: f64) -> Result<&mut Image, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewImage(width, height) -> Image,
        )
    }
}

impl Image {
    pub(crate) fn new(_: &Ui, ptr: *mut uiImage) -> Self {
        Self { ptr }
    }
}

/// An RGBA bitmap.
///
/// # Examples
///
/// ```no_run
/// // TODO
/// ```
pub struct Image {
    ptr: *mut uiImage,
}

impl Image {
    pub(crate) fn as_ptr(&self) -> *mut uiImage {
        self.ptr
    }

    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn push(&self, pixels: &mut [u8], width: u16, height: u16, byte_stride: u16) {
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
