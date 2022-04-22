// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
pub mod control;
pub mod prelude;
pub mod window;

pub use control::Control;
pub use window::Window;

mod ffi;

use prelude::*;
use std::{borrow::Cow, ptr};

#[derive(Debug)]
pub enum Error {
    AlreadyInitedLibui,
    LibuiInit { cause: String },
    ConvertString(std::ffi::NulError),
    LibuiNewWindow,
}

pub struct Ui {
    _inner: (),
}

impl Ui {
    pub fn new() -> Result<Self, Error> {
        use std::sync::Once;

        static INIT: Once = Once::new();

        let mut result = Err(Error::AlreadyInitedLibui);
        INIT.call_once(|| unsafe {
            result = Self::init_unchecked();
        });

        result.map(|_| Self { _inner: () })
    }

    unsafe fn init_unchecked() -> Result<(), Error> {
        let mut init_options = uiInitOptions { Size: 0 };
        let err_msg = uiInit(ptr::addr_of_mut!(init_options));

        let result = ffi::result_from_err_msg(err_msg);
        if result.is_err() {
            uiFreeInitError(err_msg);
        }

        result.map_err(|cause| Error::LibuiInit { cause })
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        unsafe {
            uiQuit();
        }
    }
}

impl Ui {
    pub fn create_window(
        // TODO: Should this be `mut`?
        &self,
        title: Cow<str>,
        width: u16,
        height: u16,
        has_menubar: bool,
    ) -> Result<Window, Error> {
        let title = ffi::create_c_string(title).map_err(Error::ConvertString)?;
        let window = unsafe {
            uiNewWindow(title.as_ptr(), width.into(), height.into(), has_menubar.into())
        };

        ffi::result_from_obj(window)
            .map_err(|_| Error::LibuiNewWindow)
            .map(|inner| unsafe { Window::from_ptr(inner) })
    }
}

impl Ui {
    pub fn start(&mut self) {
        unsafe { uiMain() }
    }
}
