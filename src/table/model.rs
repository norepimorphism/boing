// SPDX-License-Identifier: MPL-2.0

use crate::prelude::*;

impl Ui {
    // pub fn create_table_model(&self) -> Model {
    //     call_libui_new_fn!(
    //         ui: self,
    //         fn: uiNewTableModel() -> Model,
    //     )
    // }
}

pub struct Model {
    ptr: *mut uiTableModel,
}

impl Model {
    pub(crate) fn as_ptr(&self) -> *mut uiTableModel {
        todo!()
    }
}
