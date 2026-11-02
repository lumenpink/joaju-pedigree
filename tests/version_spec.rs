use std::ffi::CStr;

#[test]
fn test_rust_version_constant() {
    assert!(
        !joaju_pedigree::VERSION.is_empty(),
        "VERSION const cannot be empty"
    );
    assert!(
        joaju_pedigree::VERSION.starts_with("0.") || joaju_pedigree::VERSION.starts_with("1."),
        "VERSION should start with SemVer major digit"
    );
}

#[test]
fn test_c_abi_version_export() {
    let c_ptr = joaju_pedigree::ffi::joaju_pedigree_version();
    assert!(
        !c_ptr.is_null(),
        "joaju_pedigree_version pointer must not be null"
    );
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    let version_str = c_str.to_str().expect("Valid UTF-8 C string");
    assert_eq!(version_str, joaju_pedigree::VERSION);
}
