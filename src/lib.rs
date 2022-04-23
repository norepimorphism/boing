// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod area;
pub mod boks;
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
pub use boks::Boks;
pub use checkbox::Checkbox;
pub use combobox::Combobox;
pub use control::Control;
pub use form::Form;
pub use grid::Grid;
pub use group::Group;
pub use image::Image;
pub use label::Label;
pub use progress_bar::ProgressBar;
pub use slider::Slider;
pub use spinbox::Spinbox;
pub use tab::Tab;
pub use table::Table;
pub use window::Window;

mod ffi;
#[macro_use]
mod macros;

use prelude::*;
use std::ptr;

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

        let result = ffi::result_from_err_msg(err_msg);
        if result.is_err() {
            uiFreeInitError(err_msg);
        }

        result.map_err(|cause| Error::LibuiInit { cause })
    }
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
