// SPDX-License-Identifier: MPL-2.0

//! [`Window`].

use std::{os::raw::c_void, ptr};

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Window`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_window(
        &self,
        title: impl AsRef<str>,
        width: u16,
        height: u16,
        has_menubar: bool,
        should_quit_on_close: bool,
    ) -> Result<&mut Window, crate::Error> {
        let title = make_cstring!(title.as_ref());
        let window = call_libui_new_fn!(
            ui: self,
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

                // Returning `true` tells *libui-ng* to destroy the window, which isn't necessary in
                // this case as `uiQuit` accomplishes that.
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
            unsafe extern "C" fn on_closing(window: *mut uiWindow, _: *mut c_void) -> i32 {
                // SAFETY: We can't return `true` here, as that would cause the window to be
                // destroyed, invalidating the window's inner pointer and allowing use-after-frees
                // to occur. Instead, we will simply hide the window. This is kind of a silly
                // solution, but it works.

                uiControlHide(window.cast());

                false.into()
            }

            unsafe {
                uiWindowOnClosing(window.as_ptr(), Some(on_closing), ptr::null_mut());
            }
        }

        Ok(window)
    }
}

def_subcontrol!(
    docs: "
        An application window.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Window,
    handle: uiWindow,
    cb_fns: [
        on_content_size_changed(),
        on_closing(),
    ],
);

impl<'ui> Window<'ui> {
    bind_text_fn!(
        docs: "
            The title of this window.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: title(),
            raw_fn: raw_title(),
            as_ptr_fn: title_ptr(),
        },
        libui: { fn: uiWindowTitle() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the title of this window.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_title(title) -> () },
        libui: { fn: uiWindowSetTitle() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when the content size of this window changes.

            This callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Window<'ui>,
            handle: uiWindow,
            fn: on_content_size_changed(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiWindowOnContentSizeChanged(),
            cb: { sig: () -> () },
        },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this window is requested to close.

            This callback is set to exit the application by default when the option
            `should_quit_on_close = true` is passed to [`Ui::create_window`].

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Window<'ui>,
            handle: uiWindow,
            fn: on_closing(),
            cb: {
                sig: f -> (),
                map: |_| {
                    // SAFETY: Returning `true` here will unleash chaos. See [`Ui::create_window`]
                    // for the reason why.
                    i32::from(false)
                },
            },
        },
        libui: {
            fn: uiWindowOnClosing(),
            cb: { sig: () -> i32 },
        },
    );

    bind_bool_fn!(
        docs: "
            Determines if this window is fullscreen.

            Windows are not fullscreen by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_fullscreen() -> bool },
        libui: { fn: uiWindowFullscreen() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this window is fullscreen.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_fullscreen(value: bool) },
        libui: { fn: uiWindowSetFullscreen() },
    );

    bind_bool_fn!(
        docs: "
            Determines if this window is borderless.

            Windows are bordered by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_borderless() -> bool },
        libui: { fn: uiWindowBorderless() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this window is borderless.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_borderless(value: bool) },
        libui: { fn: uiWindowSetBorderless() },
    );

    bind_add_child_fn!(
        docs: "
            Sets the child control of this window.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_child<'ui>(child) },
        libui: { fn: uiWindowSetChild() },
    );

    bind_bool_fn!(
        docs: "
            Determines if this window has margins.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_margined() -> bool },
        libui: { fn: uiWindowMargined() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this window has margins.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_margined(value: bool) },
        libui: { fn: uiWindowSetMargined() },
    );

    bind_bool_fn!(
        docs: "
            Determines if this window is resizeable.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_resizeable() -> bool },
        libui: { fn: uiWindowResizeable() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this window is resizeable.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_resizeable(value: bool) },
        libui: { fn: uiWindowSetResizeable() },
    );

    bind_fn!(
        docs: "
            Sets the inner size of this window.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_content_size(width: u16, height: u16) },
        libui: { fn: uiWindowSetContentSize() },
    );

    /// The inner size of this window.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn content_size(&self) -> (u16, u16) {
        let (mut width, mut height) = (0, 0);
        unsafe {
            uiWindowContentSize(
                self.as_ptr(),
                ptr::addr_of_mut!(width),
                ptr::addr_of_mut!(height),
            );
        }

        (to_u16!(width), to_u16!(height))
    }
}

macro_rules! impl_present_fn {
    (
        $name:ident,
        $fn:ident $(,)?
    ) => {
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

impl_present_fn!(present_alert, uiMsgBox);
impl_present_fn!(present_error, uiMsgBoxError);
