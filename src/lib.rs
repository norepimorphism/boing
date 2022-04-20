// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// pub mod window;

// pub use window::Window;

use libui_ng_sys::*;
use std::{ffi::CStr, ptr};

pub struct Ui;

impl Ui {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let mut init_options = uiInitOptions { Size: 0 };
            let maybe_err_msg = uiInit(ptr::addr_of_mut!(init_options)).as_ref();

            if maybe_err_msg.is_none() {
                return Ok(Ui);
            }

            let err_msg = CStr::from_ptr(&*maybe_err_msg.unwrap());

            Err(Error::Init { msg: err_msg.to_string_lossy().to_string() })
        }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        unsafe {
            uiUninit();
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Init { msg: String }
}
