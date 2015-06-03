extern crate stegpeg;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use stegpeg::algorythms::{Algorythm,LSB};

const ORIG_IMG_PATH:  &'static str = "tests/files/lena.jpg";
const SECRET40_PATH:  &'static str = "tests/files/test40k.txt";
const SECRET100_PATH: &'static str = "tests/files/test100k.txt";
const OUT_IMG_PATH:   &'static str = "/tmp/lsb_out.jpg";

#[test]
fn test_lsb() {
  let path = Path::new(SECRET40_PATH);
  let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(err) => panic!("couldn't open {}: {}", path.display(),
                       Error::description(&err))

  };
  let mut data: Vec<u8> = vec![];

  match file.read_to_end(&mut data) {
    Ok(_) => {
      let lsb = LSB::new(&HashMap::new());
      stegpeg::encode_file(ORIG_IMG_PATH, OUT_IMG_PATH, &|coefs| {
        return lsb.enc(coefs, &data)
      })
    },
    Err(err) => panic!("couldn't write: {}", err)
  }

  let coefs = stegpeg::decode_file(OUT_IMG_PATH);
  let lsb = LSB::new(&HashMap::new());
  let new_data = match lsb.dec(&coefs) {
    Ok(new_data) => new_data,
    Err(err)     => panic!("{}", err)
  };

  assert!(new_data == data);
}

#[test]
fn test_lsb_too_long() {
  let path = Path::new(SECRET100_PATH);
  let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(err) => panic!("couldn't open {}: {}", path.display(),
                       Error::description(&err))

  };
  let mut data: Vec<u8> = vec![];

  match file.read_to_end(&mut data) {
    Ok(_) =>
      stegpeg::encode_file(ORIG_IMG_PATH, OUT_IMG_PATH, &|coefs| {
        let lsb = LSB::new(&HashMap::new());
        let res = lsb.enc(coefs, &data);
        match res {
          Ok(_)    => panic!("Encoding of a too long file should fail."),
          Err(err) => assert!(err == "Image is too small")
        }
        return res;
      }),
    Err(err) => panic!("couldn't write: {}", err)
  };
}
