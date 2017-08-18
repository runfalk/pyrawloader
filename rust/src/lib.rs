extern crate rawloader;

use rawloader::SRGBImage;
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
    pub data_len: usize,
    capacity: usize,
}

#[no_mangle]
pub unsafe extern "C" fn raw_open(path: *const c_char) -> *mut Raw {
    // This will be used to open a file later on
    let path = CStr::from_ptr(path).to_string_lossy().into_owned();
    println!("Got path {:?}", path);

    let image = rawloader::decode(&path).unwrap();

    // Image pixel data
    let srgb = image.to_srgb(500, 500).unwrap();
    let SRGBImage {mut data, width, height} = srgb;

    // Deconstructed image pixel data
    let ptr = data.as_mut_ptr();
    let len = data.len();
    let capacity = data.capacity();

    // Make sure pixel data is not de-allocated when this function terminates
    std::mem::forget(data);

    let s = Raw {
        maker: CString::new(image.make).unwrap().into_raw(),
        model: CString::new(image.model).unwrap().into_raw(),
        width: width,
        height: height,
        data: ptr,
        data_len: len,
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
    let _: Vec<u8> = Vec::from_raw_parts(buff.data, buff.data_len, buff.capacity);

    println!("Freeing {:?}", buff);
}
