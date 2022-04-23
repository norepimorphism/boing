// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::ptr;

impl Ui {
    /// Creates a new [`Window`].
    pub fn create_window(
        &mut self,
        title: impl Into<Vec<u8>>,
        width: u16,
        height: u16,
        has_menubar: bool,
    ) -> Result<Window, crate::Error> {
        call_libui_new_fn!(
            Window,
            uiNewWindow,
            make_cstring!(title).as_ptr(),
            width.into(),
            height.into(),
            has_menubar.into(),
        )
    }
}

def_subcontrol_with_ptr_ty!(Window, uiWindow);

impl Window {
    /// The title.
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

    /// Determines if the window is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { uiWindowFullscreen(self.as_ptr()) == 1 }
    }

    pub fn set_fullscreen(&mut self, value: bool) {
        unsafe { uiWindowSetFullscreen(self.as_ptr(), value as i32) }
    }

    /// Determines if the window is borderless.
    pub fn is_borderless(&self) -> bool {
        unsafe { uiWindowBorderless(self.as_ptr()) == 1 }
    }

    pub fn set_borderless(&mut self, value: bool) {
        unsafe { uiWindowSetBorderless(self.as_ptr(), value as i32) }
    }

    pub fn set_child(&mut self, child: &Control) {
        unsafe { uiWindowSetChild(self.as_ptr(), child.as_ptr()) }
    }

    /// Determines if the window is margined.
    pub fn is_margined(&self) -> bool {
        unsafe { uiWindowMargined(self.as_ptr()) == 1 }
    }

    pub fn set_margined(&mut self, value: bool) {
        unsafe { uiWindowSetMargined(self.as_ptr(), value as i32) }
    }

    /// Determines if the window is resizeable.
    pub fn is_resizeable(&self) -> bool {
        unsafe { uiWindowResizeable(self.as_ptr()) == 1 }
    }

    pub fn set_resizeable(&mut self, value: bool) {
        unsafe { uiWindowSetResizeable(self.as_ptr(), value as i32) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        use std::ops::DerefMut as _;

        unsafe { uiControlDestroy(self.deref_mut().as_ptr()) }
    }
}
