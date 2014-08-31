#![feature(globs)]

extern crate stegpeg;
extern crate libc;

//use libc;
use std::default::Default;

use stegpeg::libjpeg::*;

#[test]
fn test_write() {
  let mut image_buffer = [0 as libc::c_uchar, ..120_000];

  for i in range(1u, 120_000) {
    if i % 3 == 0 {
      image_buffer[i] = 255 as libc::c_uchar
    }
  }

  unsafe {
    let jerr = jpeg_error_mgr{ ..Default::default() };
    let mut cinfo = jpeg_compress_struct{ ..Default::default() };
    let pcinfo = &cinfo as j_compress_ptr;
    cinfo.err = jpeg_std_error(&jerr);

    let row_stride: uint;

    jpeg_create_compress_fn(&*pcinfo);

    let filename = "/tmp/test.jpeg".to_c_str();
    let outfile = libc::fopen(filename.unwrap(), "wb".to_c_str().unwrap());
    jpeg_stdio_dest(&*pcinfo, outfile);

    let image_width = 200u;
    cinfo.image_width = image_width as libc::c_uint;
    cinfo.image_height = 200;
    cinfo.input_components = 3;
    cinfo.in_color_space = 2; // RGB

    jpeg_set_defaults(&*pcinfo);
    jpeg_set_quality(&*pcinfo, 5, 1);
    jpeg_start_compress(&*pcinfo, 1);

    row_stride = image_width * 3;

    while cinfo.next_scanline < cinfo.image_height {
      let row_pointer =
        &image_buffer[cinfo.next_scanline as uint * row_stride] as JSAMPROW;
      jpeg_write_scanlines(&*pcinfo, &row_pointer as JSAMPARRAY, 1);
    }

    jpeg_finish_compress(&cinfo);
    libc::fclose(outfile);
    jpeg_destroy_compress(&cinfo);
  }
}
