// SPDX-License-Identifier: MPL-2.0

//! A safe, lightweight wrapper over *[libui-ng-sys]*.
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
//! window.set_child(&mut ui.create_pushbutton("Press Me!")?);
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
//! ## Screenshots
//!
#![cfg_attr(
    feature = "doc-images",
    cfg_attr(
        all(),
        doc = embed_doc_image::embed_image!(
            "control-gallery.basic-controls",
            "screenshots/control-gallery/basic-controls@2x.png"
        ),
        doc = embed_doc_image::embed_image!(
            "control-gallery.numbers-and-lists",
            "screenshots/control-gallery/numbers-and-lists@2x.png"
        ),
        doc = embed_doc_image::embed_image!(
            "control-gallery.data-choosers",
            "screenshots/control-gallery/data-choosers@2x.png"
        ),
    ),
)]
// This warning would be tiresome to include every time we embed an image, so it will only be used
// in the crate-level documentation.
#![cfg_attr(
    not(feature = "doc-images"),
    doc = indoc::indoc! {"
        **Doc images are not enabled**. Compile *boing* with feature `doc-images` and Rust version
        >= 1.54 to enable.
    "},
)]
//!
//! ### Windows
//!
//! ![A screenshot of the example application "libui Control Gallery" with Windows 11 widgets,
//! focused on a tab labeled "Basic Controls".][control-gallery.basic-controls]
//! ![A screenshot of the example application "libui Control Gallery" with Windows 11 widgets,
//! focused on a tab labeled "Numbers and Lists".][control-gallery.numbers-and-lists]
//! ![A screenshot of the example application "libui Control Gallery" with Windows 11 widgets,
//! focused on a tab labeled "Data Choosers".][control-gallery.data-choosers]
//!
//! ### macOS
//!
//! ### Linux
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
mod axis;
mod checkbox;
pub mod color;
mod combobox;
mod control;
pub mod font;
pub mod form;
mod grid;
mod group;
mod image;
mod label;
pub mod menu;
mod path;
mod prelude;
mod progress_bar;
mod pushbutton;
pub mod radio_buttons;
mod separator;
mod slider;
mod spinbox;
mod tab;
mod table;
mod ui;
mod window;

use std::fmt;

pub use area::Area;
pub use axis::Axis;
pub use checkbox::Checkbox;
pub use color::Color;
pub use combobox::Combobox;
pub use control::Control;
pub use font::{Font, Picker as FontPicker};
pub use form::Form;
pub use grid::Grid;
pub use group::Group;
pub use image::Image;
pub use label::Label;
pub use menu::{Item as MenuItem, Menu};
pub use path::Path;
pub use progress_bar::ProgressBar;
pub use pushbutton::Pushbutton;
pub use radio_buttons::{Item as RadioItem, RadioButtons};
pub use separator::Separator;
pub use slider::Slider;
pub use spinbox::Spinbox;
pub use tab::Tab;
pub use table::Table;
pub use ui::Ui;
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
