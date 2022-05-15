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
            pub fn $self_fn(&self) -> Result<&mut MultilineTextEntry, crate::Error> {
                call_libui_new_fn!(
                    ui: self,
                    fn: $libui_fn() -> MultilineTextEntry,
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
    self: { fn: create_wrapping_multiline_text_entry() },
    libui: { fn: uiNewMultilineEntry() },
);

impl_text_entry!(
    docs: "
        # Examples

        ```no_run
        // TODO
        ```
    ",
    self: { fn: create_non_wrapping_multiline_text_entry() },
    libui: { fn: uiNewNonWrappingMultilineEntry() },
);

def_subcontrol!(
    docs: "
        A box in which multiple lines of text are displayed.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: MultilineTextEntry,
    handle: uiMultilineEntry,
    cb_fns: [ on_changed() ],
);

impl<'ui> MultilineTextEntry<'ui> {
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
        libui: { fn: uiMultilineEntryText() },
    );

    bind_set_text_fn!(
        docs: "
            Replaces the text displayed in this entry.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) -> () },
        libui: { fn: uiMultilineEntrySetText() },
    );

    bind_set_text_fn!(
        docs: "
            Appends the given text to this entry.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: push_text(text) -> () },
        libui: { fn: uiMultilineEntryAppend() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when the text within this entry changes.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: MultilineTextEntry<'ui>,
            handle: uiMultilineEntry,
            fn: on_changed(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiMultilineEntryOnChanged(),
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
        libui: { fn: uiMultilineEntryReadOnly() },
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
        libui: { fn: uiMultilineEntrySetReadOnly() },
    );
}
