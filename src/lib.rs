// The foreign function interface which exposes this library to non-Rust 
// languages.

pub mod db_manager;
use db_manager::DBManager;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::{ptr, slice};

#[no_mangle]
pub async unsafe extern "C" fn db_manager_init(db_path: *const c_char) -> *mut DBManager {
    if db_path.is_null() {
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(db_path);

    let db_path_as_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let manager = DBManager::init(db_path_as_str).await;
    match manager {
        Ok(m) =>  Box::into_raw(Box::new(m)),
        Err(_) => return ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn db_manager_destroy(manager: *mut DBManager) {
    if !manager.is_null() {
        drop(Box::from_raw(manager));
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_last_error(manager: *mut DBManager, buffer: *mut c_char, length: c_int) -> c_int {
    let manager = Box::from_raw(manager);
    let error = manager.get_last_error();

    if error.len() >= length as usize {
        return -1;
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    ptr::copy_nonoverlapping(
        error.as_ptr(),
        buffer.as_mut_ptr(),
        error.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error.len()] = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_last_error_length(manager: *mut DBManager) -> c_int{
    let manager = Box::from_raw(manager);
    manager.get_last_error().len() as c_int + 1
}
