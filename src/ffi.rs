// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{borrow::Cow, ffi::{CStr, CString}, os::raw::c_char};

pub fn create_c_string(string: Cow<str>) -> Result<CString, std::ffi::NulError> {
    match string {
        Cow::Borrowed(it) => CString::new(it),
        Cow::Owned(it) => CString::new(it),
    }
}

pub fn result_from_err_msg(err_msg: *const c_char) -> Result<(), String> {
    if err_msg.is_null() {
        Ok(())
    } else {
        let err_msg = unsafe { CStr::from_ptr(err_msg) }
            .to_string_lossy()
            .into_owned();

        Err(err_msg)
    }
}

pub fn result_from_obj<T>(obj: *mut T) -> Result<*mut T, ()> {
    if obj.is_null() {
        Err(())
    } else {
        Ok(obj)
    }
}
