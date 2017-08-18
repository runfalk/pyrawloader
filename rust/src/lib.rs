use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Raw {
    pub maker: *mut c_char,
    pub model: *mut c_char,
    pub width: usize,
    pub height: usize,
    pub data: *mut u8,
    len: usize,
    capacity: usize,
}

#[no_mangle]
pub unsafe extern "C" fn raw_open(path: *const c_char) -> *mut Raw {
    // This will be used to open a file later on
    let path = CStr::from_ptr(path);
    println!("Got path {:?}", path);

    // Image pixel data
    let mut data = vec![0x00, 0xff, 0xff];

    // Deconstructed image pixel data
    let ptr = data.as_mut_ptr();
    let len = data.len();
    let capacity = data.capacity();

    // Make sure pixel data is not de-allocated when this function terminates
    std::mem::forget(data);

    let s = Raw {
        maker: CString::new("Nikon").unwrap().into_raw(),
        model: CString::new("D7000").unwrap().into_raw(),
        width: 1,
        height: 1,
        data: ptr,
        len: len,
        capacity: capacity,
    };

    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub unsafe extern "C" fn raw_free(buff: *mut Raw) {
    let buff = Box::from_raw(buff);
    // Retake ownership of the pointers to ensure proper de-allocation
    let _ = CString::from_raw(buff.maker);
    let _ = CString::from_raw(buff.model);
    let _: Vec<u8> = Vec::from_raw_parts(buff.data, buff.len, buff.capacity);

    println!("Freeing {:?}", buff);
}
