// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Window`].

use crate::prelude::*;
use std::{os::raw::c_void, ptr};

impl<'ui> Ui<'ui> {
    /// Creates a new [`Window`].
    pub fn create_window(
        &self,
        title: impl AsRef<str>,
        width: u16,
        height: u16,
        has_menubar: bool,
        should_quit_on_close: bool,
    ) -> Result<&mut Window<'ui>, crate::Error> {
        let title = make_cstring!(title.as_ref());
        let window = call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_window,
            fn: uiNewWindow(
                title.as_ptr(),
                width.into(),
                height.into(),
                has_menubar.into(),
            ) -> Window,
        )?;

        if should_quit_on_close {
            unsafe extern "C" fn on_closing(_: *mut uiWindow, _: *mut c_void) -> i32 {
                // When the window recieves an event to close, call `uiQuit`.
                uiQuit();

                false.into()
            }

            unsafe extern "C" fn on_should_quit(_: *mut c_void) -> i32 {
                true.into()
            }

            unsafe {
                let window = window.as_ptr();
                uiWindowOnClosing(window, Some(on_closing), ptr::null_mut());
                uiOnShouldQuit(Some(on_should_quit), ptr::null_mut());
            }
        } else {
            unsafe extern "C" fn on_closing(_: *mut uiWindow, _: *mut c_void) -> i32 {
                true.into()
            }

            unsafe {
                uiWindowOnClosing(window.as_ptr(), Some(on_closing), ptr::null_mut());
            }
        }

        Ok(window)
    }
}

def_subcontrol!(
    ty: Window,
    handle: uiWindow,
    cb_fns: [
        on_content_size_changed(),
        on_closing() -> bool,
    ],
);

impl<'ui> Window<'ui> {
    bind_text_fn!(
        docs: "The title of this window.",
        self: {
            fn: title,
            raw_fn: raw_title,
            as_ptr_fn: title_ptr,
        },
        libui: {
            fn: uiWindowTitle(),
        },
    );

    bind_set_text_fn!(
        docs: "Sets the title of this window.",
        set_title,
        title,
        uiWindowSetTitle,
    );

    bind_callback_fn!(
        docs: "Sets a callback for when the content size of this window changes.",
        self: {
            ty: Window<'ui>,
            handle: uiWindow,
            fn: on_content_size_changed(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiWindowOnContentSizeChanged(),
            cb: {
                sig: () -> (),
            },
        },
    );

    bind_callback_fn!(
        docs: "Sets a callback for when this window is requested to close.",
        self: {
            ty: Window<'ui>,
            handle: uiWindow,
            fn: on_closing(),
            cb: {
                sig: should_close -> bool,
                map: |it: bool| { i32::from(it) },
            },
        },
        libui: {
            fn: uiWindowOnClosing(),
            cb: {
                sig: () -> i32,
            },
        },
    );

    bind_bool_fn!(
        docs: "Determines if this window is fullscreen.",
        is_fullscreen,
        uiWindowFullscreen,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this window is fullscreen.",
        set_fullscreen,
        uiWindowSetFullscreen,
    );

    bind_bool_fn!(
        docs: "Determines if this window is borderless.",
        is_borderless,
        uiWindowBorderless,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this window is borderless.",
        set_borderless,
        uiWindowSetBorderless,
    );

    bind_add_child_fn!(
        docs: "Sets the child control of this window.",
        self: {
            fn: set_child<'ui>,
            child: child,
        },
        libui: {
            fn: uiWindowSetChild,
        }
    );

    bind_bool_fn!(
        docs: "Determines if this window has margins.",
        is_margined,
        uiWindowMargined,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this window has margins.",
        set_margined,
        uiWindowSetMargined,
    );

    bind_bool_fn!(
        docs: "Determines if this window is resizeable.",
        is_resizeable,
        uiWindowResizeable,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this window is resizeable.",
        set_resizeable,
        uiWindowSetResizeable,
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

    pub fn set_content_size(&self, width: u16, height: u16) {
        unsafe {
            uiWindowSetContentSize(self.as_ptr(), width.into(), height.into());
        }
    }
}

macro_rules! impl_present_fn {
    ($name:ident, $fn:ident $(,)?) => {
        impl Window<'_> {
            pub fn $name(
                &self,
                title: impl AsRef<str>,
                desc: impl AsRef<str>,
            ) -> Result<(), crate::Error> {
                let title = make_cstring!(title.as_ref());
                let desc = make_cstring!(desc.as_ref());
                unsafe { $fn(self.as_ptr(), title.as_ptr(), desc.as_ptr()) };

                Ok(())
            }
        }
    };
}

impl_present_fn!(present_alert, uiMsgBox,);

impl_present_fn!(present_error, uiMsgBoxError,);
