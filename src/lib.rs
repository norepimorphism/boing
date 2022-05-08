// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A safe, lightweight wrapper over *[libui-ng-sys]*.
//!
//! ## Notice
//!
//! This crate is by no means production-ready. See the [README] for progress notes.
//!
//! ## Examples
//!
//! ```no_run
//! Ui::run(|ui| {
//!     let menu = ui.create_menu("File")?;
//!     menu.append_quit_item()?;
//!
//!     let window = ui.create_window("Hello World!", 200, 200, true, true)?;
//!     window.set_child(ui.create_button("Press Me!")?);
//!     window.show();
//! })?;
//! ```
//!
//! For more examples, including a control gallery, see the *[examples]* subdirectory.
//!
//! [libui-ng-sys]: https://crates.io/crates/libui-ng-sys
//! [README]: https://github.com/norepimorphism/boing/tree/main/README.md
//! [examples]: https://github.com/norepimorphism/boing/tree/main/examples

#![allow(non_upper_case_globals)]
#![feature(concat_idents)]

#[macro_use]
mod macros;

pub mod area;
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
pub mod ui;
pub mod unibox;
pub mod window;

pub use area::Area;
pub use button::Button;
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
pub use ui::Ui;
pub use unibox::UniBox;
pub use window::Window;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    AlreadyInitedLibui,
    ConvertCString(std::str::Utf8Error),
    ConvertRustString(std::ffi::NulError),
    LibuiFn {
        name: &'static str,
        cause: Option<String>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyInitedLibui => {
                write!(f, "*libui-ng* is already initialized")
            }
            Self::ConvertCString(e) => {
                write!(f, "failed to convert C string to Rust string: {}", e)
            }
            Self::ConvertRustString(e) => {
                write!(f, "failed to convert Rust string to C string: {}", e)
            }
            Self::LibuiFn { name, cause } => {
                write!(f, "*libui-ng* function `{}` failed", name)?;
                if let Some(cause) = cause {
                    write!(f, ": {}", cause)?;
                }

                Ok(())
            }
        }
    }
}
