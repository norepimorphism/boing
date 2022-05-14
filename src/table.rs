// SPDX-License-Identifier: MPL-2.0

//! [`Table`].

mod model;

pub use model::Model;

use crate::prelude::*;

impl Ui {
    // pub fn create_table_model(&self) -> Model {
    //     call_libui_new_fn!(
    //         ui: self,
    //         fn: uiNewTableModel() -> Model,
    //     )
    // }
}

#[derive(Clone, Copy, Debug)]
pub enum RowBackgroundColor {
    Default,
    SameAsColumn { index: u16 },
}

impl RowBackgroundColor {
    fn into_param(self) -> i32 {
        match self {
            Self::Default => -1,
            Self::SameAsColumn { index } => index.into(),
        }
    }
}

impl Ui {
    pub fn create_table<'ui>(
        &'ui self,
        model: Model,
        row_bg: RowBackgroundColor,
    ) -> Result<&'ui mut Table, crate::Error> {
        let mut params = uiTableParams {
            // TODO: `model` is dropped at the end of scope, so this will always be a
            // use-after-free.
            Model: model.as_ptr(),
            RowBackgroundColorModelColumn: row_bg.into_param(),
        };

        call_libui_new_fn!(
            ui: self,
            fn: uiNewTable(std::ptr::addr_of_mut!(params)) -> Table,
        )
    }
}

def_subcontrol!(
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Table,
    handle: uiTable,
);

impl Table {
    bind_bool_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_header_visible() -> bool },
        libui: { fn: uiTableHeaderVisible() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_header_visible(value: bool) },
        libui: { fn: uiTableHeaderSetVisible() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: width_of_column(index: u16) -> i32 },
        libui: { fn: uiTableColumnWidth() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_width_of_column(index: u16, width: u16) },
        libui: { fn: uiTableColumnSetWidth() },
    );
}
