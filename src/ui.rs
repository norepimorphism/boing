// SPDX-License-Identifier: MPL-2.0

//! [`Ui`].

use std::{ffi::CStr, os::raw::c_char, ptr};

use crate::prelude::*;

impl Ui<'_> {
    /// Runs *libui-ng*.
    pub fn run(mut main: impl FnMut(&Self)) -> Result<(), crate::Error> {
        let ui = Self::new()?;
        main(&ui);
        unsafe { uiMain() };

        Ok(())
    }

    /// Creates a new [`Ui`].
    fn new() -> Result<Self, crate::Error> {
        use std::sync::Once;

        static INIT: Once = Once::new();

        let mut result = Err(crate::Error::AlreadyInitedLibui);
        INIT.call_once(|| unsafe {
            // TODO: Calling `Self::init_unchecked` inside `INIT.call_once` prevents the caller from
            // retrying `Ui::new` if it fails the first time.
            result = Self::init_unchecked();
        });

        result.map(|_| Self::default())
    }

    unsafe fn init_unchecked() -> Result<(), crate::Error> {
        // TODO: What is `uiInitOptions`? What does the `Size` field mean?
        let mut init_options = uiInitOptions { Size: 0 };

        let err_msg = uiInit(ptr::addr_of_mut!(init_options));

        let result = Self::result_from_err_msg(err_msg);
        if result.is_err() {
            // It's OK to free `err_msg` now because we first copied its contents into `result`.
            uiFreeInitError(err_msg);
        }

        if let Err(ref cause) = result {
            // For some reason, on Windows, *libui-ng* will sometimes return an error message
            // starting with the below string, which clearly indicates that no error has occurred.
            // We catch this special case and ignore it.
            if cause.starts_with("error initializing libui: initializing Common Controls; code 0") {
                return Ok(());
            }
        }

        result.map_err(|cause| {
            crate::Error::LibuiFn {
                name: "uiInit",
                cause: Some(cause),
            }
        })
    }

    fn result_from_err_msg(err_msg: *const c_char) -> Result<(), String> {
        if err_msg.is_null() {
            Ok(())
        } else {
            let err_msg = unsafe { CStr::from_ptr(err_msg) }
                .to_string_lossy()
                .into_owned();

            Err(err_msg)
        }
    }
}

macro_rules! def_ui {
    (
        $(
            {
                field: $field:ident,
                fn: $fn:ident() -> $ty:ident $(,)?
            }
        ),* $(,)?
    ) => {
        /// A graphical user interface provided by *libui-ng*.
        pub struct Ui<'ui> {
            $(
                $field: typed_arena::Arena<$crate::$ty<'ui>>
            ),*
        }

        impl<'ui> Ui<'ui> {
            // This is intentionally *not* `Default::default`, as we don't want to export this
            // publicly.
            fn default() -> Self {
                Self {
                    $(
                        $field: typed_arena::Arena::new()
                    ),*
                }
            }

            $(
                #[allow(clippy::mut_from_ref)]
                pub(crate) fn $fn<'a>(&'a self, value: $crate::$ty<'ui>) -> &'a mut $crate::$ty<'ui> {
                    let result = self.$field.alloc(value);
                    tracing::debug!("Allocated new `{}` @ {:#?}", stringify!($ty), result.as_ptr());

                    result
                }
            )*
        }
    };
}

def_ui![
    {
        field: windows,
        fn: alloc_window() -> Window,
    },
    {
        field: areas,
        fn: alloc_area() -> Area,
    },
    {
        field: comboboxes,
        fn: alloc_combobox() -> Combobox,
    },
    {
        field: forms,
        fn: alloc_form() -> Form,
    },
    {
        field: grids,
        fn: alloc_grid() -> Grid,
    },
    {
        field: groups,
        fn: alloc_group() -> Group,
    },
    {
        field: tabs,
        fn: alloc_tab() -> Tab,
    },
    {
        field: tables,
        fn: alloc_table() -> Table,
    },
    {
        field: uniboxes,
        fn: alloc_unibox() -> UniBox,
    },
    {
        field: buttons,
        fn: alloc_button() -> Button,
    },
    {
        field: checkboxes,
        fn: alloc_checkbox() -> Checkbox,
    },
    {
        field: images,
        fn: alloc_image() -> Image,
    },
    {
        field: labels,
        fn: alloc_label() -> Label,
    },
    {
        field: progress_bars,
        fn: alloc_progress_bar() -> ProgressBar,
    },
    {
        field: sliders,
        fn: alloc_slider() -> Slider,
    },
    {
        field: spinboxes,
        fn: alloc_spinbox() -> Spinbox,
    },
    {
        field: menus,
        fn: alloc_menu() -> Menu,
    },
    {
        field: menu_items,
        fn: alloc_menu_item() -> MenuItem,
    },
];
