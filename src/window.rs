// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::{os::raw::c_void, ptr};

impl Ui {
    /// Creates a new [`Window`].
    pub fn create_window(
        &mut self,
        title: impl AsRef<str>,
        width: u16,
        height: u16,
        has_menubar: bool,
        should_quit_on_close: bool,
    ) -> Result<Window, crate::Error> {
        let title = make_cstring!(title.as_ref());
        let window = call_libui_new_fn!(
            self,
            true,
            Window,
            uiNewWindow,
            title.as_ptr(),
            width.into(),
            height.into(),
            has_menubar.into(),
        )?;

        if should_quit_on_close {
            unsafe {
                let window = window.as_ptr();
                uiWindowOnClosing(window, Some(close_window), ptr::null_mut());
                uiOnShouldQuit(Some(quit_ui), ptr::null_mut());
            }
        }

        Ok(window)
    }
}

unsafe extern "C" fn close_window(_: *mut uiWindow, _: *mut c_void) -> i32 {
    // When the window recieves an event to close, call `uiQuit`.
    uiQuit();

    0
}

unsafe extern "C" fn quit_ui(_: *mut c_void) -> i32 {
    // TODO: I don't know why this returns 1, but that's what *libui*'s Control Gallery does.
    1
}

def_subcontrol!(Window, uiWindow);

impl Window {
    bind_text_fn!(
        title,
        raw_title,
        title_ptr,
        uiWindowTitle,
    );

    bind_set_text_fn!(
        set_title,
        title,
        uiWindowSetTitle,
    );

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

    bind_callback_fn!(
        on_content_size_changed,
        (),
        uiWindow,
        uiWindowOnContentSizeChanged,
    );

    bind_callback_fn!(
        on_closing,
        i32,
        uiWindow,
        uiWindowOnClosing,
    );

    bind_bool_fn!(
        is_fullscreen,
        uiWindowFullscreen,
    );

    bind_set_bool_fn!(
        set_fullscreen,
        uiWindowSetFullscreen,
    );

    bind_bool_fn!(
        is_borderless,
        uiWindowBorderless,
    );

    bind_set_bool_fn!(
        set_borderless,
        uiWindowSetBorderless,
    );

    pub fn set_child(&mut self, ui: &mut Ui, mut child: impl DerefMut<Target = Control>) {
        ui.release_control(child.deref_mut().as_ptr());
        unsafe { uiWindowSetChild(self.as_ptr(), child.as_ptr()) };
    }

    bind_bool_fn!(
        is_margined,
        uiWindowMargined,
    );

    bind_set_bool_fn!(
        set_margined,
        uiWindowSetMargined,
    );

    bind_bool_fn!(
        is_resizeable,
        uiWindowResizeable,
    );

    bind_set_bool_fn!(
        set_resizeable,
        uiWindowSetResizeable,
    );
}
