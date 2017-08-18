extern crate rawloader;

use rawloader::SRGBImage;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: *mut u8,
    pub data_len: usize,
    capacity: usize,
}

#[no_mangle]
pub unsafe extern "C" fn raw_open(path: *const c_char, max_width: usize, max_height: usize) -> Image {
    // This will be used to open a file later on
    let path = CStr::from_ptr(path).to_string_lossy().into_owned();

    // Decode raw image
    let image = rawloader::decode(&path).unwrap();

    // Extract rgb pixel data
    let srgb = image.to_srgb(max_width, max_height).unwrap();
    let SRGBImage {mut data, width, height} = srgb;

    // Deconstructed image pixel data
    let ptr = data.as_mut_ptr();
    let len = data.len();
    let capacity = data.capacity();

    // Make sure pixel data is not de-allocated when this function terminates
    std::mem::forget(data);

    Image {
        width: width,
        height: height,
        data: ptr,
        data_len: len,
        capacity: capacity,
    }
}

#[no_mangle]
pub unsafe extern "C" fn raw_free(buff: Image) {
    // Retake ownership of the pointers to ensure proper de-allocation
    let _: Vec<u8> = Vec::from_raw_parts(buff.data, buff.data_len, buff.capacity);
}
