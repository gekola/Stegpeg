#![feature(convert)]

extern crate stegpeg;

use std::env;
use std::io::Read;
use std::io::Write;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() == 3 {
    let mut data: Vec<u8> = vec![];

    std::io::stdin().read_to_end(&mut data).unwrap();
    stegpeg::encode_file(args[1].as_str(), args[2].as_str(), (&|coefs: &mut stegpeg::CoefArray| {
      return stegpeg::algorythms::lsb::enc(coefs, &data);
    }));
  } else if args.len() == 2 {
    let coefs = stegpeg::decode_file(args[1].as_str());
    let data = stegpeg::algorythms::lsb::dec(&coefs).unwrap();
    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(&data[..]);
    let _ = stdout.flush();
  } else {
    println!("jpeg_common_struct: {}", std::mem::size_of::<stegpeg::libjpeg::jpeg_common_struct>());
    println!("jpeg_compress_struct: {}", std::mem::size_of::<stegpeg::libjpeg::jpeg_compress_struct>());
    println!("jpeg_decompress_struct: {}", std::mem::size_of::<stegpeg::libjpeg::jpeg_decompress_struct>());
    println!("jpeg_decompress_struct: {}", std::mem::size_of::<stegpeg::libjpeg::jpeg_error_mgr>());
  }
}
