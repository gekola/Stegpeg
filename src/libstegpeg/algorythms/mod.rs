use algorythms::common::{AlgOptions,CoefArray};
pub use algorythms::lsb::LSB;
pub use algorythms::jsteg::JSteg;

pub mod common;

mod lsb;
mod jsteg;

pub trait Algorythm {
  fn new(opts: AlgOptions) -> Self;
  fn enc<'a>(&self, coefs: &'a mut CoefArray, data: &Vec<u8>)
             -> Result<&'a CoefArray, &'a str>;
  fn dec(&self, coefs: &CoefArray) -> Result<Vec<u8>, &str>;
}
