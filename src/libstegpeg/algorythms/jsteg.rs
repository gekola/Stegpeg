extern crate num;

use algorythms::Algorythm;
use algorythms::common::{AlgOptions,CoefArray};

use std::cmp;

pub struct JSteg {
  awidth: usize,
  maxbwidth: usize
}

const DEFAULT_AWIDTH: usize = 5;

impl Algorythm for JSteg {
  fn new(options: AlgOptions) -> Self {
    let awidth = match options.get("awidth") {
      Some(awidth_any) => {
        match awidth_any.downcast_ref::<usize>() {
          Some(awidth) => *awidth,
          None => DEFAULT_AWIDTH
        }
      },
      None => DEFAULT_AWIDTH
    };
    let maxbwidth = num::pow(2, awidth);
    return JSteg { awidth: awidth, maxbwidth: maxbwidth };
  }

  fn enc<'a>(&self, coefs: &'a mut CoefArray, data: &Vec<u8>)
             -> Result<&'a CoefArray, &'a str> {
    let size = data.len() as u32;
    let szw: usize = (1..self.maxbwidth).fold(0, |acc, item|
      if size & (0x1 << item) != 0x0 {
        cmp::max(acc, item)
      } else {
        acc
      });

    // Current bit
    let mut ind = 0usize;

    // Components
    for x in coefs.iter_mut() {
      // Rows
      for x in x.iter_mut() {
        // Blocks
        for x in x.iter_mut() {
          for i in 0..x.len() {
            if x[i] & 0x1 != x[i] {
              if ind < self.awidth {
                x[i] &= !0x1 as i16;
                x[i] |= ((szw >> ind) & 1) as i16;
              } else if ind < self.awidth + szw + 1 as usize {
                x[i] &= !0x1 as i16;
                x[i] |= ((size >> (ind - self.awidth)) & 1) as i16;
              } else if ind < self.awidth + szw + 1 + size as usize * 8 as usize {
                let cor_ind = ind - self.awidth - szw - 1 as usize;
                let base = cor_ind / 8;
                let offset = cor_ind % 8;

                x[i] &= !0x1 as i16;
                x[i] |= ((data[base] >> offset) & 1) as i16;
              }
              ind += 1;
            }
          }
        }
      }
    };
    if ind < self.awidth + size as usize * 8 + szw as usize {
      return Err("Image is too small");
    } else {
      return Ok(coefs);
    }
  }

  fn dec(&self, coefs: & CoefArray) -> Result<Vec<u8>, &str> {
    let mut data = Vec::<u8>::new();
    // Size bits width (in bits)
    let mut szw = 0usize;
    // Payload size (in bytes)
    let mut size = 0usize;

    // Current bit
    let mut ind = 0usize;

    // Components
    for x in coefs.iter() {
      // Rows
      for x in x.iter() {
        // Blocks
        for x in x.iter() {
          for p in x.iter() {
            if *p & 0x1 != *p {
              if ind < self.awidth {
                // Read payload size block width
                szw |= (*p as usize & 1) << ind;
              } else if ind < self.awidth + szw + 1 {
                // Read payload size
                size |= (*p as usize & 1) << (ind - self.awidth);
              } else if ind < self.awidth + szw + 1 + size * 8 {
                // Read  payload data
                let cor_ind = ind - self.awidth - szw - 1 as usize;
                let base = cor_ind / 8;
                let offset = cor_ind % 8;

                if offset == 0 {
                  data.push(0);
                }
                data[base] |= (*p as u8 & 1) << offset;

                if base == size-1 && offset == 7 {
                  return Ok(data)
                }
              }
              ind += 1;
            }
          }
        }
      }
    }
    return Err("Cannot extract data, maybe there is none.");
  }
}
