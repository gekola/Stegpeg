          /*
    ,***   *            *
    *      *   ,**.  ,**.  ,**.  ,**.  ,**.
     **.  ***  * .*  *  *  *  *  * .*  *  *
       *   *   *`    *  *  * ,*  *`    *  *
    ***    `*   ***   `*.  *`     ***   `*.
                        *  *              *
                     ***   *           ***/


// #![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_id = "stegpeg"]

#![allow(ctypes)]

// pub use libjpeg;

pub mod libjpeg;

#[no_mangle]
pub extern fn printme(me: uint) {
  println!("HI: {}", me);
}
