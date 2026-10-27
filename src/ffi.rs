//! C-ABI Foreign Function Interface (FFI) for `joaju-pedigree`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// C-ABI function to render pedigree markdown code blocks to SVG.
///
/// Returns `0` on success and populates `out_format` and `out_svg`.
/// Returns `-1` on error and populates `out_error` if non-null.
///
/// # Safety
/// Pointers must be valid or null as documented. The caller is responsible for
/// freeing any non-null strings returned via [`joaju_pedigree_free_string`].
#[no_mangle]
pub unsafe extern "C" fn joaju_pedigree_render(
    raw_code: *const c_char,
    forced_format: *const c_char,
    out_format: *mut *mut c_char,
    out_svg: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if !out_format.is_null() {
        *out_format = ptr::null_mut();
    }
    if !out_svg.is_null() {
        *out_svg = ptr::null_mut();
    }
    if !out_error.is_null() {
        *out_error = ptr::null_mut();
    }

    if raw_code.is_null() {
        if !out_error.is_null() {
            if let Ok(c_err) = CString::new("raw_code pointer is null") {
                *out_error = c_err.into_raw();
            }
        }
        return -1;
    }

    let code_str = match CStr::from_ptr(raw_code).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(format!("UTF-8 error in raw_code: {}", e)) {
                    *out_error = c_err.into_raw();
                }
            }
            return -1;
        }
    };

    let fmt_str_opt = if forced_format.is_null() {
        None
    } else {
        match CStr::from_ptr(forced_format).to_str() {
            Ok(s) if !s.is_empty() => Some(s),
            _ => None,
        }
    };

    match crate::block::render_pedigree(code_str, fmt_str_opt) {
        Ok((fmt, svg)) => {
            if !out_format.is_null() {
                if let Ok(c_fmt) = CString::new(fmt) {
                    *out_format = c_fmt.into_raw();
                }
            }
            if !out_svg.is_null() {
                if let Ok(c_svg) = CString::new(svg) {
                    *out_svg = c_svg.into_raw();
                }
            }
            0
        }
        Err(err) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(err) {
                    *out_error = c_err.into_raw();
                }
            }
            -1
        }
    }
}

/// Frees a string previously allocated by the C-ABI FFI.
///
/// # Safety
/// `ptr` must point to a string allocated via `CString::into_raw` or be null.
#[no_mangle]
pub unsafe extern "C" fn joaju_pedigree_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}
