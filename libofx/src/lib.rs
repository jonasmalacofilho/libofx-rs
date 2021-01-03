use libc::{c_char, c_int};
use libofx_sys::{LibofxContextPtr, LibofxFileFormat};

extern "C" {
    fn libofx_get_new_context_noexcept() -> LibofxContextPtr;

    fn libofx_proc_file_noexcept(
        libofx_context: LibofxContextPtr,
        p_filename: *const c_char,
        ftype: LibofxFileFormat,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use libofx_sys::*;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        let sample = CString::new("../docs/samples/santander.ofx").unwrap();
        unsafe {
            ofx_show_position = 0;
            ofx_ERROR_msg = 1;
            ofx_INFO_msg = 1;
            let ctx = libofx_get_new_context_noexcept();
            assert_ne!(ctx as usize, 0);
            let ret = libofx_proc_file_noexcept(ctx, sample.as_ptr(), LibofxFileFormat::AUTODETECT);
            libofx_free_context(ctx);
            assert_eq!(ret, 0);
        }
    }
}
