extern crate stegpeg;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use stegpeg::algorythms::lsb;

const ORIG_IMG_PATH: &'static str = "tests/files/lena.jpg";
const SECRET_PATH:   &'static str = "tests/files/test.txt";
const OUT_IMG_PATH:  &'static str = "/tmp/out.jpg";

#[test]
fn test_lsb() {
  let path = Path::new(SECRET_PATH);
  let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(err) => panic!("couldn't open {}: {}", path.display(),
                       Error::description(&err))

  };
  let mut data: Vec<u8> = vec![];

  match file.read_to_end(&mut data) {
    Ok(_) => {
      stegpeg::encode_file(ORIG_IMG_PATH, OUT_IMG_PATH, &|coefs| {
        return lsb::enc(coefs, &data);
      });
    }
    Err(err) => panic!("couldn't write: {}", err)
  }

  let coefs = stegpeg::decode_file(OUT_IMG_PATH);
  let new_data = match stegpeg::algorythms::lsb::dec(&coefs) {
    Ok(new_data) => new_data,
    Err(err) => panic!("{}", err)
  };

  assert!(new_data == data);
}
