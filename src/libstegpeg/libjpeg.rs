#![allow(non_camel_case_types)]

pub mod libjpeg {

  extern crate libc;
  use self::libc::{c_int, c_short, c_uint, c_void};

  type jpeg_compress_struct = c_void; // FIXME: import the structure
  type j_compress_ptr = *jpeg_compress_struct;
  type boolean = c_int;
  type JSAMPLE = *c_short;
  type JSAMPROW = *JSAMPLE;
  type JSAMPARRAY = *JSAMPROW;
  type JDIMENSION = c_uint;

  #[link(name = "jpeg")]
  #[allow(dead_code)]
  extern {
    fn jpeg_abort_compress(cinfo: j_compress_ptr);
    fn jpeg_create_compress(cinfo: j_compress_ptr);
    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    fn jpeg_finish_compress(cinfo: j_compress_ptr);
    fn jpeg_set_defaults(cinfo: j_compress_ptr);
    fn jpeg_set_quality(cinfo: j_compress_ptr, quality: c_int,
                        force_baseline: boolean);
    fn jpeg_start_compress(cinfo: j_compress_ptr, write_all_tables: boolean);
    fn jpeg_write_scanlines(cinfo: j_compress_ptr, scanlines: JSAMPARRAY,
                            num_lines: JDIMENSION) -> JDIMENSION;

  }

}
