// SPDX-License-Identifier: MPL-2.0

//! A safe, lightweight wrapper over *[libui-ng-sys]*.
//!
//! ## Notice
//!
//! This crate is by no means production-ready. See the [README] for progress notes.
//!
//! ## Background
//!
//! *[libui]* is a C library that provides a neutral interface to native GUI technologies (e.g.,
//! windows, widgets) on major OSes. *[libui-ng]* is the "next generation" of *libui*, developed and
//! maintained separately. *libui-ng-sys* provides Rust bindings to *libui-ng*, and *boing* is a
//! safe yet fairly unopinionated layer on top of *libui-ng-sys*.
//!
//! ## Terminology
//!
//! In the context that *boing* uses them, the terms "widget" and "control" are *not*
//! interchangeable. A **widget** is an interactive visual element, while **controls** are a
//! specific subset of widgets that implement `DerefMut<Target = boing::Control>`. In particular,
//! all widgets are controls except for [`Menu`] and [`MenuItem`].
//!
//! ## Usage
//!
//! To get started with *boing*, see [`Ui`].
//!
//! ## Examples
//!
//! ```no_run
//! # fn main() -> Result<(), boing::Error> {
//! # use boing::Ui;
//! #
//! let ui = Ui::new()?;
//!
//! // Append a drop-down menu labeled "File" to the menubar of all windows created with
//! // `has_menubar` set to `true`; see [`Ui::create_window`] for more information.
//! let file_menu = ui.create_menu("File")?;
//! // Append a menu item labeled "Quit" (in English) to the previously-created file menu. This
//! // "Quit" item will exit the application when clicked.
//! file_menu.append_quit_item(&ui)?;
//!
//! // Create a 200x200 pixel window titled "Hello World!" with a menubar that exits the application
//! // when closed.
//! let window = ui.create_window("Hello World!", 200, 200, true, true)?;
//! // Create a button labeled "Press Me!" and set it as the main child control of the
//! // previously-created window.
//! window.set_child(&mut ui.create_button("Press Me!")?);
//! // Present the window to the user. Calling this method is necessary for the window to appear at
//! // all.
//! window.show();
//!
//! // Enter the UI event loop. As [`Ui::run`] borrows immutably, this can be called again.
//! ui.run();
//! #
//! # Ok(())
//! # }
//! ```
//!
//! For more examples, including a control gallery, see the *[examples]* subdirectory.
//!
//! [libui-ng-sys]: https://crates.io/crates/libui-ng-sys
//! [README]: https://github.com/norepimorphism/boing/tree/main/README.md
//! [libui]: https://github.com/andlabs/libui
//! [libui-ng]: https://github.com/libui-ng/libui-ng
//! [examples]: https://github.com/norepimorphism/boing/tree/main/examples

// All *libui-ng-sys* exports violate Rust's naming convention.
#![allow(non_upper_case_globals)]

#[macro_use]
mod macros;

pub mod area;
pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod control;
pub mod font_button;
pub mod form;
pub mod grid;
pub mod group;
pub mod image;
pub mod label;
pub mod menu;
pub mod path;
mod prelude;
pub mod progress_bar;
pub mod radio_buttons;
pub mod separator;
pub mod slider;
pub mod spinbox;
pub mod tab;
pub mod table;
pub mod ui;
pub mod unibox;
pub mod window;

use std::fmt;

pub use area::Area;
pub use button::Button;
pub use checkbox::Checkbox;
pub use combobox::Combobox;
pub use control::Control;
pub use font_button::FontButton;
pub use form::Form;
pub use grid::Grid;
pub use group::Group;
pub use image::Image;
pub use label::Label;
pub use menu::{Item as MenuItem, Menu};
pub use path::Path;
pub use progress_bar::ProgressBar;
pub use radio_buttons::RadioButtons;
pub use separator::Separator;
pub use slider::Slider;
pub use spinbox::Spinbox;
pub use tab::Tab;
pub use table::Table;
pub use ui::Ui;
pub use unibox::UniBox;
pub use window::Window;

/// The error type returned by fallible *boing* functions.
#[derive(Debug)]
pub enum Error {
    /// *libui-ng* is already initialized.
    ///
    /// This error is returned from [`Ui::new`] when called multiple times. Please ensure that
    /// [`Ui::new`] is invoked exactly once in your application.
    AlreadyInitedLibui,
    /// A C string failed to be converted to a Rust string.
    ConvertCString(std::str::Utf8Error),
    /// A Rust string failed to be converted to a C string.
    ConvertRustString(std::ffi::NulError),
    /// A *libui-ng* function failed.
    LibuiFn {
        /// The name of the function that failed.
        name: &'static str,
        /// The cause, if any, of the failure.
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
