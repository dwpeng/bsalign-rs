#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unsafe_op_in_unsafe_fn)]
pub mod bindings {
    include!("bindings.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        unsafe {
            let version = bindings::bsalign_version();
            assert!(!version.is_null(), "Version string is null");
            let version_str = std::ffi::CStr::from_ptr(version)
                .to_str()
                .expect("Failed to convert version to string");
            println!("bsalign version: {}", version_str);
            assert_eq!(version_str, "1.2.1");
        }
    }
}
