use std::any::Any;
use std::collections::HashMap;

use libjpeg::JBLOCK;

pub type CoefArray = Vec<Vec<Vec<JBLOCK>>>;
pub type AlgOptions<'a, 'b> = &'a HashMap<&'b str, Box<Any>>;
