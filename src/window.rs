// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::{os::raw::c_void, ptr};

impl Ui {
    /// Creates a new [`Window`].
    pub fn create_window(
        &mut self,
        title: impl Into<Vec<u8>>,
        width: u16,
        height: u16,
        has_menubar: bool,
    ) -> Result<Window, crate::Error> {
        let title = make_cstring!(title);
        let window = call_libui_new_fn!(
            self,
            Window,
            uiNewWindow,
            title.as_ptr(),
            width.into(),
            height.into(),
            has_menubar.into(),
        )?;

        unsafe {
            let window = window.as_ptr();
            uiWindowOnClosing(window, Some(close_window), ptr::null_mut());
            uiOnShouldQuit(Some(quit_ui), window.cast());
        }

        Ok(window)
    }
}

unsafe extern "C" fn close_window(_: *mut uiWindow, _: *mut c_void) -> i32 {
    // When the window recieves an event to close, call `uiQuit`.
    uiQuit();

    0
}

unsafe extern "C" fn quit_ui(window: *mut c_void) -> i32 {
    // When `uiQuit` is called, destroy the main window.
    uiControlDestroy(window.cast());

    // TODO: I don't know why this returns 1, but that's what *libui*'s Control Gallery does.
    1
}

def_subcontrol!(Window, uiWindow);

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
            );
        }
    }

    /// Determines if the window is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { uiWindowFullscreen(self.as_ptr()) == 1 }
    }

    pub fn set_fullscreen(&mut self, value: bool) {
        unsafe { uiWindowSetFullscreen(self.as_ptr(), value.into()) };
    }

    /// Determines if the window is borderless.
    pub fn is_borderless(&self) -> bool {
        unsafe { uiWindowBorderless(self.as_ptr()) == 1 }
    }

    pub fn set_borderless(&mut self, value: bool) {
        unsafe { uiWindowSetBorderless(self.as_ptr(), value.into()) };
    }

    pub fn set_child(&mut self, ui: &mut Ui, mut child: impl DerefMut<Target = Control>) {
        ui.remove_control(child.deref_mut().as_ptr());
        unsafe { uiWindowSetChild(self.as_ptr(), child.as_ptr()) };
    }

    /// Determines if the window is margined.
    pub fn is_margined(&self) -> bool {
        unsafe { uiWindowMargined(self.as_ptr()) == 1 }
    }

    pub fn set_margined(&mut self, value: bool) {
        unsafe { uiWindowSetMargined(self.as_ptr(), value.into()) };
    }

    /// Determines if the window is resizeable.
    pub fn is_resizeable(&self) -> bool {
        unsafe { uiWindowResizeable(self.as_ptr()) == 1 }
    }

    pub fn set_resizeable(&mut self, value: bool) {
        unsafe { uiWindowSetResizeable(self.as_ptr(), value.into()) };
    }
}
