// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Image`].
    pub fn create_image(&mut self, width: f64, height: f64) -> Result<Image, crate::Error> {
        call_libui_new_fn!(self, Image, uiNewImage, width, height)
    }
}

def_subcontrol!(Image, uiImage);
