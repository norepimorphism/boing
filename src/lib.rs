// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(concat_idents)]

#[macro_use]
mod macros;

pub mod area;
pub mod boxx;
pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod control;
pub mod form;
pub mod grid;
pub mod group;
pub mod image;
pub mod label;
pub mod menu;
pub mod prelude;
pub mod progress_bar;
pub mod slider;
pub mod spinbox;
pub mod tab;
pub mod table;
pub mod window;

pub use area::Area;
pub use boxx::Boxx;
pub use checkbox::Checkbox;
pub use combobox::Combobox;
pub use control::Control;
pub use form::Form;
pub use grid::Grid;
pub use group::Group;
pub use image::Image;
pub use label::Label;
pub use menu::{Menu, Item as MenuItem};
pub use progress_bar::ProgressBar;
pub use slider::Slider;
pub use spinbox::Spinbox;
pub use tab::Tab;
pub use table::Table;
pub use window::Window;

use prelude::*;
use std::{ffi::CStr, os::raw::c_char, ptr};

#[derive(Debug)]
pub enum Error {
    AlreadyInitedLibui,
    ConvertString(std::ffi::NulError),
    LibuiFn {
        name: &'static str,
        cause: Option<String>,
    },
}

impl Ui {
    /// Creates a new [`Ui`].
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

        let result = Self::result_from_err_msg(err_msg);
        if result.is_err() {
            uiFreeInitError(err_msg);
        }

        result.map_err(|cause| Error::LibuiFn {
            name: "uiInit",
            cause: Some(cause),
        })
    }

    fn result_from_err_msg(err_msg: *const c_char) -> Result<(), String> {
        if err_msg.is_null() {
            Ok(())
        } else {
            let err_msg = unsafe { CStr::from_ptr(err_msg) }
                .to_string_lossy()
                .into_owned();

            Err(err_msg)
        }
    }
}

pub struct Ui {
    _inner: (),
}

impl Drop for Ui {
    fn drop(&mut self) {
        unsafe { uiQuit() }
    }
}

impl Ui {
    pub fn start(&mut self) {
        unsafe { uiMain() }
    }
}
