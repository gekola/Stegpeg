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

#![feature(collections)]
#![feature(libc)]

pub mod libjpeg;
pub mod algorythms;

extern crate libc;

use std::default::Default;
use std::ffi::CString;
use std::mem::transmute;
use libjpeg::*;
pub type CoefArray = algorythms::common::CoefArray;

pub fn encode_file(in_path: &str,
                   out_path: &str,
                   tr: &Fn(&mut CoefArray) -> Result<&CoefArray, &str>) {
  let mut inputinfo = jpeg_decompress_struct{ ..Default::default() };
  let mut outputinfo = jpeg_compress_struct{ ..Default::default() };
  let mut jerr = jpeg_error_mgr{ ..Default::default() };
  //let mut coef_buffers: [JBLOCKARRAY; MAX_COMPONENTS] = [std::ptr::null_mut::<JBLOCKROW>(); MAX_COMPONENTS];
  //let mut row_ptrs: [JBLOCKARRAY; MAX_COMPONENTS] = [std::ptr::null_mut::<JBLOCKROW>(); MAX_COMPONENTS];
  let input_file: *mut libc::FILE;
  let output_file: *mut libc::FILE;

  unsafe {
    input_file = libc::fopen(CString::new(in_path).unwrap().as_ptr(), "rb".as_ptr() as *const i8);
    jpeg_create_decompress_fn(&mut inputinfo);

    inputinfo.err = jpeg_std_error(&mut jerr);

    jpeg_stdio_src(&mut inputinfo, input_file);

    let (mut coefs, orig_raw_coefs) = get_coefficients(&mut inputinfo).unwrap();

    match tr(&mut coefs) {
      Ok(coefs) => {
        output_file = libc::fopen(CString::new(out_path).unwrap().as_ptr(), "wb".as_ptr() as *const i8);
        outputinfo.err = jpeg_std_error(&mut jerr);
        jpeg_create_compress_fn(&mut outputinfo);
        jpeg_stdio_dest(&mut outputinfo, output_file);
        jpeg_copy_critical_parameters(&mut inputinfo, &mut outputinfo);

        set_coefficients(&mut outputinfo, coefs, orig_raw_coefs);

        jpeg_finish_compress(&mut outputinfo);
        jpeg_destroy_compress(&mut outputinfo);
        libc::fclose(output_file);
      }
      Err(err) =>
        println!("{}", err)
      }
    jpeg_finish_decompress(&mut inputinfo);
    jpeg_destroy_decompress(&mut inputinfo);
    /* Close files */
    libc::fclose(input_file);
  }
}

pub fn decode_file(in_path: &str) -> CoefArray {
  let mut inputinfo = jpeg_decompress_struct{ ..Default::default() };
  let mut jerr = jpeg_error_mgr{ ..Default::default() };
  let input_file: *mut libc::FILE;

  unsafe {
    input_file = libc::fopen(CString::new(in_path).unwrap().as_ptr(), "rb".as_ptr() as *const i8);
    jpeg_create_decompress_fn(&mut inputinfo);

    inputinfo.err = jpeg_std_error(&mut jerr);

    jpeg_stdio_src(&mut inputinfo, input_file);

    let (coefs, _) = get_coefficients(&mut inputinfo).unwrap();

    jpeg_finish_decompress(&mut inputinfo);
    jpeg_destroy_decompress(&mut inputinfo);
    /* Close files */
    libc::fclose(input_file);

    return coefs;
  }
}


pub fn get_coefficients(mut inputinfo: &mut jpeg_decompress_struct)
                        -> Result<(CoefArray, *const jvirt_barray_ptr), u8> {
  let mut res: CoefArray;
  let coef_arrays;

  unsafe {
    jpeg_read_header(inputinfo, 1);

    coef_arrays = jpeg_read_coefficients(inputinfo);

    res = (0..inputinfo.num_components as isize).map(|compnum| {
      let height =(*inputinfo.comp_info.offset(compnum)).height_in_blocks;
      let width = (*inputinfo.comp_info.offset(compnum)).width_in_blocks;
      (0..height).map(|rownum| {
        let row_ptrs = match (*inputinfo.mem).access_virt_barray {
          Some(access_virt_barray) =>
            access_virt_barray(transmute::<_,j_common_ptr>(&mut inputinfo),
                               *coef_arrays.offset(compnum as isize),
                               rownum as u32, 1 as JDIMENSION, 0),
          None => std::ptr::null_mut()
        };

        Vec::from_raw_buf(*row_ptrs, width as usize)
        //(0..width).
        //  map(|blocknum| *().offset(blocknum as isize)).
        //  collect::<Vec<JBLOCK>>()
      }).collect::<Vec<Vec<JBLOCK>>>()
    }).collect::<CoefArray>();
  }
  return Ok((res, coef_arrays));
}

pub fn set_coefficients(mut outputinfo: &mut jpeg_compress_struct,
                        coefficients: &CoefArray,
                        raw_coefs: *const jvirt_barray_ptr) {
  //println!("=== Set coeffs ===");

  unsafe {
    //println!("{} x {} x {}", coefficients.len(), coefficients[0].len(), coefficients[0][0].len());
    for ci in 0..coefficients.len() as isize {
      let height = coefficients[ci as usize].len() as u32;
      let width = coefficients[ci as usize][0].len() as u32;

      for i in 0..height as usize {
        let row_ptr = match (*outputinfo.mem).access_virt_barray {
          Some(access_virt_barray) =>
            access_virt_barray(transmute::<_,j_common_ptr>(&mut outputinfo),
                               //&mut outputinfo as j_common_ptr,
                               *raw_coefs.offset(ci),
                               i as u32, 1 as JDIMENSION, 1),
          None => std::ptr::null_mut()
        };

        //print!("(c: {}, h: {}): [", ci, i, );
        for j in 0..width as usize {
          *(*row_ptr).offset(j as isize) = coefficients[ci as usize][i][j];
          //print!("{} ", (coefficients[ci as usize][i][j])[0]);
        }
        //println!("]");
      }
    }
    jpeg_write_coefficients(outputinfo, raw_coefs);
  }
}

pub fn encode_file_cpy(in_path: String, out_path: String) {
  let mut inputinfo = jpeg_decompress_struct{ ..Default::default() };
  let mut outputinfo = jpeg_compress_struct{ ..Default::default() };
  let mut jerr = jpeg_error_mgr{ ..Default::default() };
  let mut coef_arrays;
  let mut coef_buffers: [JBLOCKARRAY; MAX_COMPONENTS] = [std::ptr::null_mut::<JBLOCKROW>(); MAX_COMPONENTS];
  let mut row_ptrs: [JBLOCKARRAY; MAX_COMPONENTS] = [std::ptr::null_mut::<JBLOCKROW>(); MAX_COMPONENTS];
  let input_file: *mut libc::FILE;
  let output_file: *mut libc::FILE;

  unsafe {
    input_file = libc::fopen(CString::new(in_path).unwrap().as_ptr(), "rb".as_ptr() as *const i8);
    output_file = libc::fopen(CString::new(out_path).unwrap().as_ptr(), "wb".as_ptr() as *const i8);
    inputinfo.err = jpeg_std_error(&mut jerr);
    jpeg_create_decompress_fn(&mut inputinfo);

    outputinfo.err = jpeg_std_error(&mut jerr);
    jpeg_create_compress_fn(&mut outputinfo);

    jpeg_stdio_src(&mut inputinfo, input_file);
    jpeg_stdio_dest(&mut outputinfo, output_file);

    jpeg_read_header(&mut inputinfo, 1);
    /* Allocate memory for reading out DCT coeffs */
    for compnum in 0..inputinfo.num_components as isize {
      coef_buffers[compnum as usize] =
        match (*inputinfo.mem).alloc_barray {
          Some(alloc_barray) =>
            alloc_barray(transmute::<_,j_common_ptr>(&mut inputinfo),
                         JPOOL_IMAGE as i32,
                         (*inputinfo.comp_info.offset(compnum)).width_in_blocks,
                         (*inputinfo.comp_info.offset(compnum)).height_in_blocks),
          None => std::ptr::null_mut()
        };
    }
    /* Read input file as DCT coeffs */
    coef_arrays = jpeg_read_coefficients(&mut inputinfo);
    /* Copy compression parameters from the input file to the output file */
    jpeg_copy_critical_parameters(&mut inputinfo, &mut outputinfo);
  }

  /* Copy DCT coeffs to a new array */
  let num_components = inputinfo.num_components;
  let mut width_in_blocks  = vec![];
  let mut height_in_blocks = vec![];

  unsafe {
    for compnum in 0..num_components as usize {
      height_in_blocks.push((*inputinfo.comp_info.offset(compnum as isize)).height_in_blocks as i32);
      width_in_blocks.push((*inputinfo.comp_info.offset(compnum as isize)).width_in_blocks as i32);
      for rownum in 0..height_in_blocks[compnum] {
        row_ptrs[compnum] = match (*inputinfo.mem).access_virt_barray {
          Some(access_virt_barray) =>
            access_virt_barray(transmute::<_,j_common_ptr>(&mut inputinfo),
                               *coef_arrays.offset(compnum as isize),
                               rownum as u32, 1 as JDIMENSION, 0),
          None => std::ptr::null_mut()
        };
        for blocknum in 0..width_in_blocks[compnum] as isize {
          for i in 0..DCTSIZE2 {
            (*(*coef_buffers[compnum].offset(rownum as isize)).offset(blocknum))[i] =
              (*(*row_ptrs[compnum]).offset(blocknum))[i];
          }
        }
      }
    }
    /* Print out or modify DCT coefficients */
    for compnum in 0..num_components as usize {
      for rownum in 0..height_in_blocks[compnum] as isize {
        for blocknum in 0..width_in_blocks[compnum] as isize {
          print!("\n\nComponent: {}, Row:{}, Column: {}\n", compnum, rownum, blocknum);
          for i in 0..DCTSIZE2 {
            (*(*coef_buffers[compnum as usize].offset(rownum)).offset(blocknum))[i] = -(*(*coef_buffers[compnum as usize].offset(rownum)).offset(blocknum))[i];
            print!("{},", (*(*coef_buffers[compnum as usize].offset(rownum)).offset(blocknum))[i]);
          }
        }
      }
    }
    print!("\n\n");
    /* Output the new DCT coeffs to a JPEG file */
    for compnum in 0..num_components as usize {
      for rownum in 0..height_in_blocks[compnum] {
        row_ptrs[compnum] =
          match (*outputinfo.mem).access_virt_barray {
            Some(access_virt_barray) =>
              access_virt_barray(transmute::<_,j_common_ptr>(&mut outputinfo),
                                 //&mut outputinfo as j_common_ptr,
                                 *coef_arrays.offset(compnum as isize),
                                 rownum as u32, 1 as JDIMENSION, 1),
            None => std::ptr::null_mut()
          };

        for z in 0..width_in_blocks[compnum] as isize {
          *(*row_ptrs[compnum]).offset(z) =
            *(*coef_buffers[compnum].offset(rownum as isize)).offset(z);
        }
      }
    }

    /* Write to the output file */
    jpeg_write_coefficients(&mut outputinfo, coef_arrays);
    /* Finish compression and release memory */
    jpeg_finish_compress(&mut outputinfo);
    jpeg_destroy_compress(&mut outputinfo);
    jpeg_finish_decompress(&mut inputinfo);
    jpeg_destroy_decompress(&mut inputinfo);
    /* Close files */
    libc::fclose(input_file);
    libc::fclose(output_file);
  }
}
