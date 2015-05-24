extern crate gcc;

fn main() {
  gcc::compile_library("libjpeglib_macrofuns.a", &["c_src/jpeglib_macrofuns.c"]);
}
