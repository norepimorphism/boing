// SPDX-License-Identifier: MPL-2.0

use crate::prelude::*;

macro_rules! impl_text_entry {
    (
        docs: $docs:literal,
        self: { fn: $self_fn:ident() $(,)? },
        libui: { fn: $libui_fn:ident() $(,)? } $(,)?
    ) => {
        impl Ui {
            #[doc = indoc::indoc!($docs)]
            pub fn $self_fn(&self) -> Result<&mut TextEntry, crate::Error> {
                call_libui_new_fn!(
                    ui: self,
                    fn: $libui_fn() -> TextEntry,
                )
            }
        }
    };
}

impl_text_entry!(
    docs: "
        # Examples

        ```no_run
        // TODO
        ```
    ",
    self: { fn: create_text_entry() },
    libui: { fn: uiNewEntry() },
);

impl_text_entry!(
    docs: "
        # Examples

        ```no_run
        // TODO
        ```
    ",
    self: { fn: create_password_text_entry() },
    libui: { fn: uiNewPasswordEntry() },
);

impl_text_entry!(
    docs: "
        # Examples

        ```no_run
        // TODO
        ```
    ",
    self: { fn: create_search_text_entry() },
    libui: { fn: uiNewSearchEntry() },
);

def_subcontrol!(
    docs: "
        A box in which single- or multi-line text is displayed.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: TextEntry,
    handle: uiEntry,
    cb_fns: [ on_changed() ],
);

impl<'ui> TextEntry<'ui> {
    bind_text_fn!(
        docs: "
            The text displayed in this entry.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: text(),
            raw_fn: raw_text(),
            as_ptr_fn: text_ptr(),
        },
        libui: { fn: uiEntryText() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the text displayed in this entry.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) -> () },
        libui: { fn: uiEntrySetText() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when the text within this entry changes.

            This callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: TextEntry<'ui>,
            handle: uiEntry,
            fn: on_changed(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiEntryOnChanged(),
            cb: { sig: () -> () },
        },
    );

    bind_bool_fn!(
        docs: "
            Determines if this entry is read-only.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_read_only() -> bool },
        libui: { fn: uiEntryReadOnly() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this entry is read-only.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_read_only(value: bool) },
        libui: { fn: uiEntrySetReadOnly() },
    );
}
