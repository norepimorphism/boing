// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::{ffi::CStr, os::raw::c_char, ptr};

impl Ui {
    /// Runs *libui*.
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
            result = Self::init_unchecked();
        });

        result.map(|_| Self {
            bump: bumpalo::Bump::new(),
        })
    }

    unsafe fn init_unchecked() -> Result<(), crate::Error> {
        let mut init_options = uiInitOptions { Size: 0 };
        let err_msg = uiInit(ptr::addr_of_mut!(init_options));

        let result = Self::result_from_err_msg(err_msg);
        if result.is_err() {
            // It's OK to free `err_msg` now because we first copied its contents into `result`.
            uiFreeInitError(err_msg);
        }

        if let Err(ref cause) = result {
            // For some reason, on Windows, *libui* will return an error message starting with the
            // below string, which clearly indicates that no error has occurred. We catch this
            // special case and ignore it.
            if cause.starts_with("error initializing libui: initializing Common Controls; code 0") {
                return Ok(());
            }
        }

        result.map_err(|cause| crate::Error::LibuiFn {
            name: "uiInit",
            cause: Some(cause),
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

pub struct Ui {
    bump: bumpalo::Bump,
}

impl Ui {
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn alloc_object<T>(&self, object: T) -> &mut T {
        self.bump.alloc(object)
    }
}
