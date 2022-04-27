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
pub mod window;

pub use area::Area;
pub use boxx::Boxx;
pub use checkbox::Checkbox;
pub use combobox::Combobox;
pub use form::Form;
pub use grid::Grid;
pub use group::Group;
pub use image::Image;
pub use label::Label;
pub use menu::{Menu, /*Item as MenuItem*/};
pub use progress_bar::ProgressBar;
pub use slider::Slider;
pub use spinbox::Spinbox;
pub use tab::Tab;
pub use table::Table;
pub use ui::{Control, Ui};
pub use window::Window;

#[derive(Debug)]
pub enum Error {
    AlreadyInitedLibui,
    ConvertString(std::ffi::NulError),
    LibuiFn {
        name: &'static str,
        cause: Option<String>,
    },
}
