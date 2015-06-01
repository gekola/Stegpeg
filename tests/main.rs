extern crate stegpeg;
extern crate libc;

use std::default::Default;
use std::ffi::CString;

use stegpeg::libjpeg;

#[test]
fn test_write() {
  let mut image_buffer = [30 as libc::c_uchar; 120_000];

  for i in 1..120_000 {
    if i % 3 != 2 {
      image_buffer[i] = ((1f32 - ((100 - (i / 600) as i16).pow(2) +
                                  (100 - (i % 600 / 3) as i16).pow(2)) as f32 /
                          20_000f32) * 255f32) as libc::c_uchar
    }
  }

  unsafe {
    let mut jerr = libjpeg::jpeg_error_mgr{ ..Default::default() };
    let mut cinfo = libjpeg::jpeg_compress_struct{ ..Default::default() };
    let pcinfo = &mut cinfo as libjpeg::j_compress_ptr;
    cinfo.err = libjpeg::jpeg_std_error(&mut jerr);

    let row_stride: usize;

    libjpeg::jpeg_create_compress_fn(pcinfo);

    let filename = CString::new("/tmp/test.jpeg");
    let outfile = libc::fopen(filename.unwrap().as_ptr(), CString::new("wb").unwrap().as_ptr());
    libjpeg::jpeg_stdio_dest(&mut *pcinfo, outfile);

    let image_width = 200usize;
    cinfo.image_width = image_width as libc::c_uint;
    cinfo.image_height = 200u32;
    cinfo.input_components = 3;
    cinfo.in_color_space = 2; // RGB

    libjpeg::jpeg_set_defaults(&mut *pcinfo);
    libjpeg::jpeg_set_quality(pcinfo, 30, 1);
    libjpeg::jpeg_start_compress(pcinfo, 1);

    row_stride = image_width * 3;

    while cinfo.next_scanline < cinfo.image_height {
      let mut row_pointer =
        &mut image_buffer[cinfo.next_scanline as usize * row_stride] as libjpeg::JSAMPROW;
      libjpeg::jpeg_write_scanlines(pcinfo, &mut row_pointer as libjpeg::JSAMPARRAY, 1);
    }

    libjpeg::jpeg_finish_compress(&mut cinfo);
    libc::fclose(outfile);
    libjpeg::jpeg_destroy_compress(&mut cinfo);
  }
}
