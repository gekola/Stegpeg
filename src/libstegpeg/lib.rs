          /*
    ,***   *            *
    *      *   ,**.  ,**.  ,**.  ,**.  ,**.
     **.  ***  * .*  *  *  *  *  * .*  *  *
       *   *   *`    *  *  * ,*  *`    *  *
    ***    `*   ***   `*.  *`     ***   `*.
                        *  *              *
                     ***   *           ***/


#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_id = "stegpeg#0.0.0"]

#![feature(macro_rules)]
#![allow(ctypes)]
// #![feature(globs)]

// pub use libjpeg::*;

pub mod libjpeg;

#[no_mangle]
pub extern fn printme(me: uint) {
  println!("HI: {}", me);
}
