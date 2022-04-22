// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Control, prelude::*};
use std::{ops::{Deref, DerefMut}, ptr};

pub struct Window(Control);

impl Window {
    pub unsafe fn from_ptr(inner: *mut uiWindow) -> Self {
        Self(Control::from_ptr(inner.cast()))
    }

    pub fn as_ptr(&self) -> *mut uiWindow {
        self.0.as_ptr().cast()
    }

    pub fn title(&self) -> String {
        todo!()
    }

    pub fn set_title(&mut self, _title: ()) {
        todo!()
    }

    pub fn content_size(&self) -> (i32, i32) {
        let (mut width, mut height) = (0, 0);
        unsafe {
            uiWindowContentSize(
                self.as_ptr(),
                ptr::addr_of_mut!(width),
                ptr::addr_of_mut!(height),
            );
       }

       (width, height)
    }

    pub fn set_content_size(&mut self, width: u16, height: u16) {
        unsafe {
            uiWindowSetContentSize(
                self.as_ptr(),
                width.into(),
                height.into(),
            )
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        unsafe { uiWindowFullscreen(self.as_ptr()) == 1 }
    }

    pub fn set_fullscreen(&mut self, value: bool) {
        unsafe { uiWindowSetFullscreen(self.as_ptr(), value as i32) }
    }

    pub fn is_borderless(&self) -> bool {
        unsafe { uiWindowBorderless(self.as_ptr()) == 1}
    }

    pub fn set_borderless(&mut self, value: bool) {
        unsafe { uiWindowSetBorderless(self.as_ptr(), value as i32) }
    }

    pub fn is_margined(&self) -> bool {
        unsafe { uiWindowMargined(self.as_ptr()) == 1 }
    }

    pub fn set_margined(&mut self, value: bool) {
        unsafe { uiWindowSetMargined(self.as_ptr(), value as i32) }
    }

    pub fn is_resizeable(&self) -> bool {
        unsafe { uiWindowResizeable(self.as_ptr()) == 1 }
    }

    pub fn set_resizeable(&mut self, value: bool) {
        unsafe { uiWindowSetResizeable(self.as_ptr(), value as i32) }
    }
}

impl Deref for Window {
    type Target = Control;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
