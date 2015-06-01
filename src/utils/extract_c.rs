#![feature(convert)]
#![feature(path_ext)]

extern crate libc;
extern crate stegpeg;

use std::env;
use std::fs::PathExt;
use std::path::Path;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() == 2 {
    let path = Path::new(args[1].as_str()).to_path_buf();
    let abs_path = path.canonicalize().unwrap();
    let mut inputinfo = stegpeg::libjpeg::jpeg_decompress_struct{ ..Default::default() };
    let mut jerr = stegpeg::libjpeg::jpeg_error_mgr{ ..Default::default() };
    let input_file: *mut libc::FILE;
    let mut coefs;
    unsafe {
      input_file = libc::fopen(abs_path.to_str().unwrap().as_ptr() as *const i8,
                               "rb".as_ptr() as *const i8);
      stegpeg::libjpeg::jpeg_create_decompress_fn(&mut inputinfo);
      inputinfo.err = stegpeg::libjpeg::jpeg_std_error(&mut jerr);

      stegpeg::libjpeg::jpeg_stdio_src(&mut inputinfo, input_file);

      coefs = stegpeg::get_coefficients(&mut inputinfo).unwrap().0;
    }

    for x in coefs {
      for x in x {
        for x in x {
          for x in x.iter() {
            println!("{}", x);
          }
        }
      }
    }

    unsafe {
      stegpeg::libjpeg::jpeg_destroy_decompress(&mut inputinfo);
      libc::fclose(input_file);
    }
  } else {
    println!("Usage: extract_c filename.jpeg");
  }
}
