use tensorflow_lite_sys as sys;
use core::ffi::CStr;

#[test]
fn check_version() {
    let version = unsafe {
        CStr::from_ptr(sys::TfLiteVersion())
    };
    let version = version.to_str().expect("to have utf-8 string");
    assert_eq!(version, "2.12.0");
}
