pub mod aead;

pub mod digest;

pub mod test;

pub mod error;

mod debug;

mod c;

mod endian;

mod polyfill;

use std::ffi::CStr;
use std::sync::Once;
static START: Once = Once::new();

#[inline]
pub fn init() {
    START.call_once(|| unsafe {
        aws_lc_sys::CRYPTO_library_init();
    });
}

#[allow(dead_code)]
unsafe fn dump_error() {
    let err = aws_lc_sys::ERR_get_error();
    let lib = aws_lc_sys::ERR_GET_LIB(err);
    let reason = aws_lc_sys::ERR_GET_REASON(err);
    let func = aws_lc_sys::ERR_GET_FUNC(err);
    let mut buffer = [0u8; 256];
    aws_lc_sys::ERR_error_string(err, buffer.as_mut_ptr().cast());
    let error_msg = CStr::from_bytes_with_nul_unchecked(&buffer);
    eprintln!(
        "Raw Error -- {:?}\nErr: {}, Lib: {}, Reason: {}, Func: {}",
        error_msg, err, lib, reason, func
    );
}

#[cfg(test)]
mod tests {
    use crate::{dump_error, init};

    #[test]
    fn test_init() {
        init();
    }

    #[test]
    fn test_dump() {
        unsafe {
            dump_error();
        }
    }
}
