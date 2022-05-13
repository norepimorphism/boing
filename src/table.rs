// SPDX-License-Identifier: MPL-2.0

//! [`Table`].

use crate::prelude::*;

impl Ui {
    pub fn create_table() -> Result<Table, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewTable() -> Table,
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
    
    bind_set_bool_fn!(
        docs: "
            # Examples
        
            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_header_visible() },
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
