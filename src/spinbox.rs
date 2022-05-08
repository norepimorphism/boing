// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Spinbox`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Spinbox`].
    pub fn create_spinbox<'a>(&'a self, min: u16, max: u16) -> Result<&'a mut Spinbox<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_spinbox,
            fn: uiNewSpinbox(min.into(), max.into()) -> Spinbox,
        )
    }
}

def_subcontrol!(
    ty: Spinbox,
    handle: uiSpinbox,
    cb_fns: [
        on_changed(),
    ],
);

impl<'ui> Spinbox<'ui> {
    bind_callback_fn!(
        docs: "Sets a callback for when this spinbox changes.",
        self: {
            ty: Spinbox<'ui>,
            handle: uiSpinbox,
            fn: on_changed(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiSpinboxOnChanged(),
            cb: {
                sig: () -> (),
            },
        },
    );
}
