#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(uppercase_variables)]

extern crate libc;

use std::default::Default;
use std::ptr;
use self::libc::{
  c_char, c_double, c_int, c_long, c_short, size_t, c_uchar, c_uint, c_void,
  FILE
};

pub type boolean = c_int;

pub type JCOEF = c_short;
pub type JOCTET = c_uchar;

pub type JSAMPLE = libc::c_uchar; // !!!
pub type JSAMPROW = *JSAMPLE;
pub type JSAMPARRAY = *JSAMPROW;
pub type JSAMPIMAGE = *JSAMPARRAY;

pub type JBLOCK = [JCOEF, ..64]; //[DCTSIZE2];
pub type JBLOCKROW = *JBLOCK;
pub type JBLOCKARRAY = *JBLOCKROW;
pub type JBLOCKIMAGE = *JBLOCKARRAY;
pub type JCOEFPTR = *JCOEF;

pub type JDIMENSION = c_uint;

#[repr(C)]
pub struct JQUANT_TBL {
  pub quantval: [u16, ..64], //[DCTSIZE2];
  pub sent_table: boolean
}

#[repr(C)]
pub struct JHUFF_TBL {
  pub bits: [u8, ..17],
  pub huffval: [u8, ..256],
  pub sent_table: boolean
}

#[repr(C)]
pub struct jpeg_component_info {
  pub component_id: c_int,
  pub component_index: c_int,
  pub h_samp_factor: c_int,
  pub v_samp_factor: c_int,
  pub quant_tbl_no: c_int,

  pub dc_tbl_no: c_int,
  pub ac_tbl_no: c_int,

  pub width_in_blocks: JDIMENSION,
  pub height_in_blocks: JDIMENSION,

  //#if JPEG_LIB_VERSION >= 70
  //DCT_h_scaled_size: c_int,
  //DCT_v_scaled_size: c_int,
  //#else
  pub DCT_scaled_size: c_int,
  //#endif

  pub downsampled_width: JDIMENSION,
  pub downsampled_height: JDIMENSION,

  pub component_needed: boolean,

  pub MCU_width: c_int,
  pub MCU_height: c_int,
  pub MCU_blocks: c_int,
  pub MCU_sample_width: c_int,
  pub last_col_width: c_int,
  pub last_row_height: c_int,

  pub quant_table: *JQUANT_TBL,

  pub dct_table: *c_void
}

#[repr(C)]
pub struct jpeg_scan_info {
  pub comps_in_scan: c_int,
  pub component_index: [c_int, ..4], //[MAX_COMPS_IN_SCAN],
  pub Ss: c_int, pub Se: c_int,
  pub Ah: c_int, pub Al: c_int
}

pub type jpeg_saved_marker_ptr = *jpeg_marker_struct;

#[repr(C)]
pub struct jpeg_marker_struct {
  pub next: jpeg_saved_marker_ptr,
  pub marker: u8,
  pub original_length: c_uint,
  pub data_length: c_uint,
  pub data: *JOCTET
}

// TODO: enum J_COLOR_SPACE
pub type J_COLOR_SPACE = c_int;
// TODO: enum J_DCT_METHOD
pub type J_DCT_METHOD = c_int;
// TODO: enum J_DITHER_MODE
pub type J_DITHER_MODE = c_int;

macro_rules! jpeg_common_fields_struct (
  ($name:ident, $($element: ident: $ty: ty),*) => {
    #[repr(C)]
    pub struct $name {
      pub err: *jpeg_error_mgr,
      pub mem: *jpeg_memory_mgr,
      pub jpeg_progress_mgr: *jpeg_progress_mgr,
      pub client_data: *c_void,
      pub is_decompressor: boolean,
      pub global_state: c_int,
      $(pub $element: $ty),*
    }
  }
  )

jpeg_common_fields_struct!(jpeg_common_struct, )

pub type j_common_ptr = *jpeg_common_struct;
pub type j_compress_ptr = *jpeg_compress_struct;
pub type j_decompress_ptr = *jpeg_decompress_struct;

jpeg_common_fields_struct!{ jpeg_compress_struct,
  dest: *jpeg_destination_mgr,

  image_width: JDIMENSION,
  image_height: JDIMENSION,
  input_components: c_int,
  in_color_space: J_COLOR_SPACE,

  input_gamma: c_double,

  //#[cfg(JPEG_LIB_VERSION >= 70)]
  // scale_num: c_uint,
  // scale_denom: c_uint,

  // jpeg_width: JDIMENSION,
  // jpeg_height: JDIMENSION,
  //#[end]

  data_precision: c_int,
  num_components: c_int,
  jpeg_color_space: J_COLOR_SPACE,

  comp_info: *jpeg_component_info,
  quant_tbl_ptrs: [*JQUANT_TBL, ..4], //[NUM_QUANT_TBLS],

  //#[cfg(JPEG_LIB_VERSION >= 70)]
  // q_scale_factor: *c_int, //[NUM_QUANT_TBLS],
  //#[end]

  dc_huff_tbl_ptrs: [*JHUFF_TBL, ..4], //[NUM_HUFF_TBLS],
  ac_huff_tbl_ptrs: [*JHUFF_TBL, ..4], //[NUM_HUFF_TBLS],

  arith_dc_L: [u8, ..16], //[NUM_ARITH_TBLS],
  arith_dc_U: [u8, ..16], //[NUM_ARITH_TBLS],
  arith_ac_K: [u8, ..16], //[NUM_ARITH_TBLS],

  num_scans: c_int,
  scan_info: *jpeg_scan_info,

  raw_data_in: boolean,
  arith_code: boolean,
  optimize_coding: boolean,
  CCIR601_sampling: boolean,
  //#if JPEG_LIB_VERSION >= 70
  // do_fancy_downsampling: boolean,
  //#endif
  smoothing_factor: c_int,
  dct_method: J_DCT_METHOD,

  restart_interval: c_uint,
  restart_in_rows: c_int,

  write_JFIF_header: boolean,
  JFIF_major_version: u8,
  JFIF_minor_version: u8,

  density_unit: u8,
  X_density: u16,
  Y_density: u16,
  write_Adobe_marker: boolean,

  next_scanline: JDIMENSION,

  progressive_mode: boolean,
  max_h_samp_factor: c_int,
  max_v_samp_factor: c_int,

  //#if JPEG_LIB_VERSION >= 70
  // min_DCT_h_scaled_size: c_int,
  // min_DCT_v_scaled_size: c_int,
  //#endif

  total_iMCU_rows: JDIMENSION,

  comps_in_scan: c_int,
  cur_comp_info: [*jpeg_component_info, ..4], //[MAX_COMPS_IN_SCAN],

  MCUs_per_row: JDIMENSION,
  MCU_rows_in_scan: JDIMENSION,

  blocks_in_MCU: c_int,
  MCU_membership: [c_int, ..10], //[C_MAX_BLOCKS_IN_MCU],

  Ss: c_int, Se: c_int, Ah: c_int, Al: c_int,

  //#if JPEG_LIB_VERSION >= 80
  // block_size: c_int,
  // natural_order: *c_int,
  // lim_Se: c_int,
  //#endif

  master: *jpeg_comp_master,
  main: *jpeg_c_main_controller,
  prep: *jpeg_c_prep_controller,
  coef: *jpeg_c_coef_controller,
  marker: *jpeg_marker_writer,
  cconvert: *jpeg_color_converter,
  downsample: *jpeg_downsampler,
  fdct: *jpeg_forward_dct,
  entropy: *jpeg_entropy_encoder,
  script_space: *jpeg_scan_info,
  script_space_size: c_int
}

impl Default for jpeg_compress_struct {
  fn default() -> jpeg_compress_struct {
    jpeg_compress_struct {
      err: ptr::null(),
      mem: ptr::null(),
      jpeg_progress_mgr: ptr::null(),
      client_data: ptr::null(),
      is_decompressor: 0,
      global_state: 0,

      dest: ptr::null(),

      image_width: 0,
      image_height: 0,
      input_components: 0,
      in_color_space: 0,

      input_gamma: 0.0,

      //#[cfg(JPEG_LIB_VERSION >= 70)]
      // scale_num: c_uint,
      // scale_denom: c_uint,

      // jpeg_width: JDIMENSION,
      // jpeg_height: JDIMENSION,
      //#[end]

      data_precision: 0,
      num_components: 0,
      jpeg_color_space: 0,

      comp_info: ptr::null(),
      quant_tbl_ptrs: [ptr::null(), ..4],

      //#[cfg(JPEG_LIB_VERSION >= 70)]
      // q_scale_factor: [c_int, ..4] //[NUM_QUANT_TBLS],
      //#[end]

      dc_huff_tbl_ptrs: [ptr::null(), ..4],
      ac_huff_tbl_ptrs: [ptr::null(), ..4],

      arith_dc_L: [0, ..16],
      arith_dc_U: [0, ..16],
      arith_ac_K: [0, ..16],

      num_scans: 0,
      scan_info: ptr::null(),

      raw_data_in: 0,
      arith_code: 0,
      optimize_coding: 0,
      CCIR601_sampling: 0,
      //#if JPEG_LIB_VERSION >= 70
      // do_fancy_downsampling: boolean,
      //#endif
      smoothing_factor: 0,
      dct_method: 0,

      restart_interval: 0,
      restart_in_rows: 0,

      write_JFIF_header: 0,
      JFIF_major_version: 0,
      JFIF_minor_version: 0,

      density_unit: 0,
      X_density: 0,
      Y_density: 0,
      write_Adobe_marker: 0,

      next_scanline: 0,

      progressive_mode: 0,
      max_h_samp_factor: 0,
      max_v_samp_factor: 0,

      //#if JPEG_LIB_VERSION >= 70
      // min_DCT_h_scaled_size: c_int,
      // min_DCT_v_scaled_size: c_int,
      //#endif

      total_iMCU_rows: 0,

      comps_in_scan: 0,
      cur_comp_info: [ptr::null(), ..4],

      MCUs_per_row: 0,
      MCU_rows_in_scan: 0,

      blocks_in_MCU: 0,
      MCU_membership: [0, ..10],

      Ss: 0, Se: 0, Ah: 0, Al: 0,

      //#if JPEG_LIB_VERSION >= 80
      // block_size: c_int,
      // natural_order: *c_int,
      // lim_Se: c_int,
      //#endif

      master: ptr::null(),
      main: ptr::null(),
      prep: ptr::null(),
      coef: ptr::null(),
      marker: ptr::null(),
      cconvert: ptr::null(),
      downsample: ptr::null(),
      fdct: ptr::null(),
      entropy: ptr::null(),
      script_space: ptr::null(),
      script_space_size: 0
    }
  }
}

jpeg_common_fields_struct!{ jpeg_decompress_struct,
  src: *jpeg_source_mgr,

  image_width: JDIMENSION,
  image_height: JDIMENSION,
  num_components: c_int,
  jpeg_color_space: J_COLOR_SPACE,

  out_color_space: J_COLOR_SPACE,

  scale_num: c_uint,
  scale_denom: c_uint,

  output_gamma: c_double,

  buffered_image: boolean,
  raw_data_out: boolean,

  dct_method: J_DCT_METHOD,
  do_fancy_upsampling: boolean,
  do_block_smoothing: boolean,

  quantize_colors: boolean,

  dither_mode: J_DITHER_MODE,
  two_pass_quantize: boolean,
  desired_number_of_colors: c_int,

  enable_1pass_quant: boolean,
  enable_external_quant: boolean,
  enable_2pass_quant: boolean,

  output_width: JDIMENSION,
  output_height: JDIMENSION,
  out_color_components: c_int,
  output_components: c_int,

  rec_outbuf_height: c_int,

  actual_number_of_colors: c_int,
  colormap: JSAMPARRAY,

  output_scanline: JDIMENSION,

  input_scan_number: c_int,
  input_iMCU_row: JDIMENSION,

  output_scan_number: c_int,
  output_iMCU_row: JDIMENSION,

  coef_bits: [*c_int, ..64], //[DCTSIZE2],

  quant_tbl_ptrs: [*JQUANT_TBL, ..4], //[NUM_QUANT_TBLS],

  dc_huff_tbl_ptrs: [*JHUFF_TBL, ..4], //[NUM_HUFF_TBLS],
  ac_huff_tbl_ptrs: [*JHUFF_TBL, ..4], //[NUM_HUFF_TBLS],

  data_precision: c_int,

  comp_info: *jpeg_component_info,

//#if JPEG_LIB_VERSION >= 80
//  is_baseline: boolean,
//#endif
  progressive_mode: boolean,
  arith_code: boolean,

  arith_dc_L: [u8, ..16], //[NUM_ARITH_TBLS],
  arith_dc_U: [u8, ..16], //[NUM_ARITH_TBLS],
  arith_ac_K: [u8, ..16], //[NUM_ARITH_TBLS],

  restart_interval: c_uint,

  saw_JFIF_marker: boolean,

  JFIF_major_version: u8,
  JFIF_minor_version: u8,
  density_unit: u8,
  X_density: u16,
  Y_density: u16,
  saw_Adobe_marker: boolean,
  Adobe_transform: u8,

  CCIR601_sampling: boolean,

  marker_list: jpeg_saved_marker_ptr,

  max_h_samp_factor: c_int,
  max_v_samp_factor: c_int,

//#if JPEG_LIB_VERSION >= 70
//  min_DCT_h_scaled_size: c_int,
//  min_DCT_v_scaled_size: c_int,
//#else
  min_DCT_scaled_size: c_int,
//#endif

  total_iMCU_rows: JDIMENSION,

  sample_range_limit: *JSAMPLE,
  comps_in_scan: c_int,
  cur_comp_info: [*jpeg_component_info, ..4], //[MAX_COMPS_IN_SCAN],

  MCUs_per_row: JDIMENSION,
  MCU_rows_in_scan: JDIMENSION,

  blocks_in_MCU: c_int,
  MCU_membership: [c_int, ..10], //[D_MAX_BLOCKS_IN_MCU],

  Ss: c_int, Se: c_int, Ah: c_int, Al: c_int,

  //#if JPEG_LIB_VERSION >= 80
  //  block_size: c_int,
  //  natural_order: *c_int,
  //  lim_Se: c_int,
  //#endif

  unread_marker: c_int,

  master: *jpeg_decomp_master,
  main: *jpeg_d_main_controller,
  coef: *jpeg_d_coef_controller,
  post: *jpeg_d_post_controller,
  inputctl: *jpeg_input_controller,
  marker: *jpeg_marker_reader,
  entropy: *jpeg_entropy_decoder,
  idct: *jpeg_inverse_dct,
  upsample: *jpeg_upsampler,
  cconvert: *jpeg_color_deconverter,
  cquantize: *jpeg_color_quantizer
}

#[repr(C)]
pub struct jpeg_error_mgr {
  pub error_exit: Option<extern fn(cinfo: j_common_ptr)>,
  pub emit_message: Option<extern fn(cinfo: j_common_ptr, msg_level: c_int)>,
  pub output_message: Option<extern fn(cinfo: j_common_ptr)>,
  pub format_message: Option<extern fn(cinfo: j_common_ptr, buffer: *c_char)>,
  pub reset_error_mgr: Option<extern fn(cinfo: j_common_ptr)>,

  pub msg_code: c_int,

  pub msg_parm: *c_char,

  pub trace_level: c_int,
  pub num_warnings: c_long,

  pub jpeg_message_table: **c_char,
  pub last_jpeg_message: c_int,

  pub addon_message_table: **c_char,
  pub first_addon_message: c_int,
  pub last_addon_message: c_int
}

impl Default for jpeg_error_mgr {
  fn default() -> jpeg_error_mgr {
    jpeg_error_mgr {
      error_exit: None, emit_message: None, output_message: None,
      format_message: None, reset_error_mgr: None,
      msg_code: 0, msg_parm: ptr::null(), trace_level: 0, num_warnings: 0,
      jpeg_message_table: ptr::null(), last_jpeg_message: 0,
      addon_message_table: ptr::null(), first_addon_message: 0,
      last_addon_message: 0
    }
  }
}

impl Default for *jpeg_error_mgr {
  fn default() -> *jpeg_error_mgr { ptr::null() }
}


#[repr(C)]
pub struct jpeg_progress_mgr {
  pub progress_monitor: Option<extern fn(cinfo: j_common_ptr)>,

  pub pass_counter: c_long,
  pub pass_limit: c_long,
  pub completed_passes: c_int,
  pub total_passes: c_int
}

impl Default for *jpeg_progress_mgr {
  fn default() -> *jpeg_progress_mgr { ptr::null() }
}

#[repr(C)]
pub struct jpeg_destination_mgr {
  pub next_output_byte: *JOCTET,
  pub free_in_buffer: size_t,

  pub init_destination: Option<extern fn(cinfo: j_compress_ptr)>,
  pub empty_output_buffer: Option<extern fn(cinfo: j_compress_ptr) -> boolean>,
  pub term_destination: Option<extern fn(cinfo: j_compress_ptr)>
}

#[repr(C)]
pub struct jpeg_source_mgr {
  pub next_input_byte: *JOCTET,
  pub bytes_in_buffer: size_t,

  pub init_source: Option<extern fn(cinfo: j_decompress_ptr)>,
  pub fill_input_buffer: Option<extern fn(cinfo: j_decompress_ptr) -> boolean>,
  pub skip_input_data:
    Option<extern fn(cinfo: j_decompress_ptr, num_bytes: c_long)>,
  pub resync_to_restart:
    Option<extern fn(cinfo: j_decompress_ptr, desired: c_int)
                     -> boolean>,
  pub term_source: Option<extern fn(cinfo: j_decompress_ptr)>
}

pub type jvirt_sarray_ptr = *jvirt_sarray_control;
pub type jvirt_barray_ptr = *jvirt_barray_control;

#[repr(C)]
pub struct jpeg_memory_mgr {
  pub alloc_small: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                    sizeofobject: size_t) -> *c_void>,

  pub alloc_large: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                    sizeofobject: size_t) -> *c_void>,

  pub alloc_sarray: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                     samplesperrow: JDIMENSION,
                                     numrows: JDIMENSION) -> JSAMPARRAY>,
  pub alloc_barray: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                     blocksperrow: JDIMENSION,
                                     numrows: JDIMENSION) -> JBLOCKARRAY>,

  pub request_virt_sarray: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                            pre_zero: boolean,
                                            samplesperrow: JDIMENSION,
                                            numrows: JDIMENSION,
                                            maxaccess: JDIMENSION)
                                            -> jvirt_sarray_ptr>,

  pub request_virt_barray: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                            pre_zero: boolean,
                                            blocksperrow: JDIMENSION,
                                            numrows: JDIMENSION,
                                            maxaccess: JDIMENSION)
                                            -> jvirt_barray_ptr>,

  pub realize_virt_arrays: Option<extern fn(cinfo: j_common_ptr)>,

  pub access_virt_sarray: Option<extern fn(cinfo: j_common_ptr,
                                           ptr: jvirt_sarray_ptr,
                                           start_row: JDIMENSION,
                                           num_rows: JDIMENSION,
                                           writable: boolean) -> JSAMPARRAY>,

  pub access_virt_barray: Option<extern fn(cinfo: j_common_ptr,
                                           ptr: jvirt_barray_ptr,
                                           start_row: JDIMENSION,
                                           num_rows: JDIMENSION,
                                           writable: boolean) -> JBLOCKARRAY>,

  pub free_pool: Option<extern fn(cinfo: j_common_ptr, pool_id: c_int)>,
  pub self_destruct: Option<extern fn(cinfo: j_common_ptr)>,

  pub max_memory_to_use: c_long,
  pub max_alloc_chunk: c_long
}

impl Default for *jpeg_memory_mgr {
  fn default() -> *jpeg_memory_mgr { ptr::null() }
}

// pub type jpeg_marker_parser_method =
//   extern fn(cinfo: j_decompress_ptr) -> boolean;

#[repr(C)]
pub struct jvirt_sarray_control { dummy: c_long }
#[repr(C)]
pub struct jvirt_barray_control { dummy: c_long }
#[repr(C)]
pub struct jpeg_comp_master { dummy: c_long }
#[repr(C)]
pub struct jpeg_c_main_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_c_prep_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_c_coef_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_marker_writer { dummy: c_long }
#[repr(C)]
pub struct jpeg_color_converter { dummy: c_long }
#[repr(C)]
pub struct jpeg_downsampler { dummy: c_long }
#[repr(C)]
pub struct jpeg_forward_dct { dummy: c_long }
#[repr(C)]
pub struct jpeg_entropy_encoder { dummy: c_long }
#[repr(C)]
pub struct jpeg_decomp_master { dummy: c_long }
#[repr(C)]
pub struct jpeg_d_main_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_d_coef_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_d_post_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_input_controller { dummy: c_long }
#[repr(C)]
pub struct jpeg_marker_reader { dummy: c_long }
#[repr(C)]
pub struct jpeg_entropy_decoder { dummy: c_long }
#[repr(C)]
pub struct jpeg_inverse_dct { dummy: c_long }
#[repr(C)]
pub struct jpeg_upsampler { dummy: c_long }
#[repr(C)]
pub struct jpeg_color_deconverter { dummy: c_long }
#[repr(C)]
pub struct jpeg_color_quantizer { dummy: c_long }

#[link(name = "jpeglib_macrofuns")]
extern "C" {
  fn jpeg_create_compress_fn(cinfo: j_compress_ptr);
  fn jpeg_create_decompress_fn(cinfo: j_decompress_ptr);
}

#[link(name = "jpeg")]
extern "C" {
  fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: c_int,
                         structsize: size_t);
  fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: c_int,
                           structsize: size_t);

  fn jpeg_abort_compress(cinfo: j_compress_ptr);

  fn jpeg_add_quant_table(cinfo: j_compress_ptr, which_tbl: c_int,
                          basic_table: *c_uint, scale_factor: c_int,
                          force_baseline: boolean);

  fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *JHUFF_TBL;
  fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *JQUANT_TBL;

  fn jpeg_default_colorspace(cinfo: j_compress_ptr);

  fn jpeg_destroy_compress(cinfo: j_compress_ptr);
  fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);

  fn jpeg_quality_scaling(quality: c_int) -> c_int;

  fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);
  fn jpeg_set_defaults(cinfo: j_compress_ptr);
  fn jpeg_set_linear_quality (cinfo: j_compress_ptr, scale_factor: c_int,
                              force_baseline: boolean);
  fn jpeg_set_quality(cinfo: j_compress_ptr, quality: c_int,
                      force_baseline: boolean);

  fn jpeg_simple_progression(cinfo: j_compress_ptr);

  fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *FILE);
   fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *FILE);

  fn jpeg_std_error(err: *jpeg_error_mgr) -> *jpeg_error_mgr;

  fn jpeg_suppress_tables (cinfo: j_compress_ptr, suppress: boolean);

  /* Main entry points for compression */
  fn jpeg_start_compress(cinfo: j_compress_ptr, write_all_tables: boolean);
  fn jpeg_write_scanlines(cinfo: j_compress_ptr, scanlines: JSAMPARRAY,
                          num_lines: JDIMENSION) -> JDIMENSION;

  fn jpeg_finish_compress(cinfo: j_compress_ptr);
  fn jpeg_write_raw_data(cinfo: j_compress_ptr, data: JSAMPIMAGE,
                         num_lines: JDIMENSION) -> JDIMENSION;

  fn jpeg_write_marker(cinfo: j_compress_ptr, marker: c_int,
                       dataptr: *JOCTET, datalen: c_uint);
  fn jpeg_write_m_header(cinfo: j_compress_ptr, marker: c_int,
                         datalen: c_uint);
  fn jpeg_write_m_byte(cinfo: j_compress_ptr, val: c_int);
  fn jpeg_write_tables(cinfo: j_compress_ptr);

  fn jpeg_read_header (cinfo: j_decompress_ptr, require_image: boolean)
                       -> c_int;

  fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
  fn jpeg_read_scanlines(cinfo: j_decompress_ptr, scanlines: JSAMPARRAY,
                         max_lines: JDIMENSION) -> JDIMENSION;
  fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
  fn jpeg_read_raw_data(cinfo: j_decompress_ptr, data: JSAMPIMAGE,
                        max_lines: JDIMENSION) -> JDIMENSION;

  fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr) -> boolean;
  fn jpeg_start_output(cinfo: j_decompress_ptr, scan_number: c_int)
                       -> boolean;
  fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean;
  fn jpeg_input_complete(cinfo: j_decompress_ptr) -> boolean;
  fn jpeg_new_colormap(cinfo: j_decompress_ptr);
  fn jpeg_consume_input(cinfo: j_decompress_ptr) -> c_int;

  //#if JPEG_LIB_VERSION >= 70
  // fn jpeg_default_qtables (cinfo: j_compress_ptr, force_baseline: boolean);
  // fn jpeg_calc_jpeg_dimensions(cinfo: j_compress_ptr);
  //#endif

  //#if JPEG_LIB_VERSION >= 80 || defined(MEM_SRCDST_SUPPORTED)
  // fn jpeg_mem_dest(cinfo: j_compress_ptr, outbuffer: **c_uchar,
  //                  outsize: *c_ulong);
  // fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *c_uchar,
  //                 insize: c_ulong);
  //#endif

}
