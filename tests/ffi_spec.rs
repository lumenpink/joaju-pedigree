use std::ffi::{CStr, CString};
use std::ptr;

#[test]
fn test_ffi_pedigree_render_success() {
    let input = CString::new("id,pai,mae,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1\n").unwrap();
    let mut out_fmt = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju_pedigree::ffi::joaju_pedigree_render(
            input.as_ptr(),
            ptr::null(),
            &mut out_fmt,
            &mut out_svg,
            &mut out_err,
        )
    };

    assert_eq!(rc, 0);
    assert!(!out_fmt.is_null());
    assert!(!out_svg.is_null());
    assert!(out_err.is_null());

    let fmt_str = unsafe { CStr::from_ptr(out_fmt).to_str().unwrap() };
    let svg_str = unsafe { CStr::from_ptr(out_svg).to_str().unwrap() };

    assert_eq!(fmt_str, "pedtools");
    assert!(svg_str.contains("<svg"));

    unsafe {
        joaju_pedigree::ffi::joaju_pedigree_free_string(out_fmt);
        joaju_pedigree::ffi::joaju_pedigree_free_string(out_svg);
    }
}

#[test]
fn test_ffi_pedigree_render_empty_error() {
    let input = CString::new("").unwrap();
    let mut out_fmt = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju_pedigree::ffi::joaju_pedigree_render(
            input.as_ptr(),
            ptr::null(),
            &mut out_fmt,
            &mut out_svg,
            &mut out_err,
        )
    };

    assert_eq!(rc, -1);
    assert!(out_fmt.is_null());
    assert!(out_svg.is_null());
    assert!(!out_err.is_null());

    let err_str = unsafe { CStr::from_ptr(out_err).to_str().unwrap() };
    assert!(!err_str.is_empty());

    unsafe {
        joaju_pedigree::ffi::joaju_pedigree_free_string(out_err);
    }
}

#[test]
fn test_ffi_pedigree_render_null_pointer() {
    let mut out_fmt = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju_pedigree::ffi::joaju_pedigree_render(
            ptr::null(),
            ptr::null(),
            &mut out_fmt,
            &mut out_svg,
            &mut out_err,
        )
    };

    assert_eq!(rc, -1);
    assert!(!out_err.is_null());
    unsafe {
        joaju_pedigree::ffi::joaju_pedigree_free_string(out_err);
    }
}
