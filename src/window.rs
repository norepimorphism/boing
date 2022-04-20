// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub struct Window {}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, has_menubar: bool) -> Self {
        unsafe {
            libui_ng_sys::uiNewWindow(title, width, height, has_menubar as i32)
        }
    }
}
