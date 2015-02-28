          /*
    ,***   *            *
    *      *   ,**.  ,**.  ,**.  ,**.  ,**.
     **.  ***  * .*  *  *  *  *  * .*  *  *
       *   *   *`    *  *  * ,*  *`    *  *
    ***    `*   ***   `*.  *`     ***   `*.
                        *  *              *
                     ***   *           ***/


#![crate_name = "stegpeg"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(libc)]
#![allow(improper_ctypes)]

// pub use libjpeg::*;

pub mod libjpeg;

#[no_mangle]
pub extern fn printme(me: usize) {
  println!("HI: {}", me);
}
