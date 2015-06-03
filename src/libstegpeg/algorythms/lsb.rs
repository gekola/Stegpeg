use algorythms::Algorythm;
use algorythms::common::{AlgOptions,CoefArray};

pub struct LSB;

impl Algorythm for LSB {
  fn new(_: AlgOptions) -> Self {
    LSB
  }

  fn enc<'a>(&self, coefs: &'a mut CoefArray, data: &Vec<u8>)
             -> Result<&'a CoefArray, &'a str> {
    let data_sz = data.len();
    let size = data_sz + 8;
    let comp =
      coefs.len() * coefs[0].len() * coefs[0].len() * coefs[0][0].len();

    if size * 8 < comp {
      let mut ind = 0usize;
      for x in coefs.iter_mut() {
        for x in x.iter_mut() {
          for x in x.iter_mut() {
            for i in 0..x.len() {
              let base = ind / 8;
              if base < size {
                x[i] &= -2 as i16;
                if base < 8 {
                  x[i] |= ((data_sz >> ind) & 1) as i16;
                } else {
                  let offset = ind % 8;
                  x[i] |= ((data[base-8] >> offset) & 1) as i16;
                }
              }
              ind += 1;
            }
          }
        }
      };
      return Ok(coefs);
    } else {
      return Err("Image is too small");
    }
  }

  fn dec(&self, coefs: &CoefArray) -> Result<Vec<u8>, &str> {
    let mut data = Vec::<u8>::new();
    let mut size = 0usize;

    let comp =
      coefs.len() * coefs[0].len() * coefs[0].len() * coefs[0][0].len();

    if comp < 8 {
      return Err("Input file is too small.");
    }

    let mut ind = 0usize;
    for x in coefs.iter() {
      for x in x.iter() {
        for x in x.iter() {
          for p in x.iter() {
            let base = ind / 8;
            if base < 8 {
              size |= (*p as usize & 1) << ind;
            }

            if ind == 64 {
              size += 8;
              if size > comp {
                return Err("Size in header is too big to be in the file.");
              }
            }

            if 8 <= base && base < size {
              let offset = ind % 8;
              if offset == 0 {
                data.push(0);
              }
              data[base - 8] |= (*p as u8 & 1) << offset;
            }

            ind += 1;
          }
        }
      }
    }

    return Ok(data);
  }
}
