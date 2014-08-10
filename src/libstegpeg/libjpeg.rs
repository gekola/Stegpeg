#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(uppercase_variables)]
pub mod libjpeg {

  extern crate libc;
  use self::libc::{
    c_char, c_double, c_int, c_long, c_short, size_t, c_uchar, c_uint, c_void,
    FILE
  };

  type boolean = c_int;

  type JCOEF = c_short;
  type JOCTET = c_uchar;

  type JSAMPLE = *c_short;
  type JSAMPROW = *JSAMPLE;
  type JSAMPARRAY = *JSAMPROW;
  type JSAMPIMAGE = *JSAMPARRAY;

  type JBLOCK = *JCOEF; //[DCTSIZE2];
  type JBLOCKROW = *JBLOCK;
  type JBLOCKARRAY = *JBLOCKROW;
  type JBLOCKIMAGE = *JBLOCKARRAY;
  type JCOEFPTR = *JCOEF;

  type JDIMENSION = c_uint;
  type J_COLOR_SPACE = c_int; //TODO: add enum
  type J_DCT_METHOD = c_int; //TODO: add enum

  struct JQUANT_TBL {
    quantval: *u16, //[DCTSIZE2];
    sent_table: boolean
  }

  struct JHUFF_TBL {
    bits: *u8, //[17],
    huffval: *u8, //[256],
    sent_table: boolean
  }

  struct jpeg_component_info {
    component_id: c_int,
    component_index: c_int,
    h_samp_factor: c_int,
    v_samp_factor: c_int,
    quant_tbl_no: c_int,

    dc_tbl_no: c_int,
    ac_tbl_no: c_int,

    width_in_blocks: JDIMENSION,
    height_in_blocks: JDIMENSION,

    //#if JPEG_LIB_VERSION >= 70
    //DCT_h_scaled_size: c_int,
    //DCT_v_scaled_size: c_int,
    //#else
    DCT_scaled_size: c_int,
    //#endif

    downsampled_width: JDIMENSION,
    downsampled_height: JDIMENSION,

    component_needed: boolean,

    MCU_width: c_int,
    MCU_height: c_int,
    MCU_blocks: c_int,
    MCU_sample_width: c_int,
    last_col_width: c_int,
    last_row_height: c_int,

    quant_table: *JQUANT_TBL,

    dct_table: *c_void
  }


  struct jpeg_scan_info {
    comps_in_scan: c_int,
    component_index: *c_int, //[MAX_COMPS_IN_SCAN],
    Ss: c_int, Se: c_int,
    Ah: c_int, Al: c_int
  }

  type jpeg_saved_marker_ptr = *jpeg_marker_struct;

  struct jpeg_marker_struct {
    next: jpeg_saved_marker_ptr,
    marker: u8,
    original_length: c_uint,
    data_length: c_uint,
    data: *JOCTET
  }

  // TODO: enum J_COLOR_SPACE

  // TODO: enum J_DCT_METHOD

  // TODO: enum J_DITHER_MODE

  macro_rules! jpeg_common_fields_struct (
    ($name:ident, $($element: ident: $ty: ty),*) => {
      struct $name {
        err: *jpeg_error_mgr,
        mem: *jpeg_memory_mgr,
        jpeg_progress_mgr: *jpeg_progress_mgr,
        client_data: *c_void,
        is_decompressor: boolean,
        global_state: c_int,
        $($element: $ty),*
      }
    }
    )

  jpeg_common_fields_struct!(jpeg_common_struct, )

  type j_common_ptr = *jpeg_common_struct;
  type j_compress_ptr = *jpeg_compress_struct;
  // type j_decompress_ptr = *jpeg_decompress_struct;

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
    quant_tbl_ptrs: *JQUANT_TBL, //[NUM_QUANT_TBLS],

    //#[cfg(JPEG_LIB_VERSION >= 70)]
    // q_scale_factor: *c_int, //[NUM_QUANT_TBLS],
    //#[end]

    dc_huff_tbl_ptrs: *JHUFF_TBL, //[NUM_HUFF_TBLS],
    ac_huff_tbl_ptrs: *JHUFF_TBL, //[NUM_HUFF_TBLS],

    arith_dc_L: *u8, //[NUM_ARITH_TBLS],
    arith_dc_U: *u8, //[NUM_ARITH_TBLS],
    arith_ac_K: *u8, //[NUM_ARITH_TBLS],

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
    cur_comp_info: *jpeg_component_info, //[MAX_COMPS_IN_SCAN],

    MCUs_per_row: JDIMENSION,
    MCU_rows_in_scan: JDIMENSION,

    blocks_in_MCU: c_int,
    MCU_membership: *c_int, //[C_MAX_BLOCKS_IN_MCU],

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

  // TODO: struct jpeg_decompress_struct

  struct jpeg_error_mgr {
    error_exit: extern fn(cinfo: j_common_ptr),
    emit_message: extern fn(cinfo: j_common_ptr, msg_level: c_int),
    output_message: extern fn(cinfo: j_common_ptr),
    format_message: extern fn(cinfo: j_common_ptr, buffer: *c_char),
    reset_error_mgr: extern fn(cinfo: j_common_ptr),

    msg_code: c_int,

    msg_parm: *c_char,

    trace_level: c_int,
    num_warnings: c_long,

    jpeg_message_table: **c_char,
    last_jpeg_message: c_int,

     addon_message_table: **c_char,
    first_addon_message: c_int,
    last_addon_message: c_int
  }

  struct jpeg_progress_mgr {
    progress_monitor: extern fn(cinfo: j_common_ptr),

    pass_counter: c_long,
    pass_limit: c_long,
    completed_passes: c_int,
    total_passes: c_int
  }

  struct jpeg_destination_mgr {
    next_output_byte: *JOCTET,
    free_in_buffer: size_t,

    init_destination: extern fn(cinfo: j_compress_ptr),
    empty_output_buffer: extern fn(cinfo: j_compress_ptr) -> boolean,
    term_destination: extern fn(cinfo: j_compress_ptr)
  }

  /*
  struct jpeg_source_mgr {
    next_input_byte: *JOCTET,
    bytes_in_buffer: size_t,

    init_source: extern fn(cinfo: j_decompress_ptr),
    fill_input_buffer: extern fn(cinfo: j_decompress_ptr) -> boolean,
    skip_input_data: extern fn(cinfo: j_decompress_ptr, long num_bytes),
    resync_to_restart: extern fn(cinfo: j_decompress_ptr, desired: c_int)
                                 -> boolean,
    term_source: extern fn(cinfo: j_decompress_ptr)
  }
  */

  type jvirt_sarray_ptr = *jvirt_sarray_control;
  type jvirt_barray_ptr = *jvirt_barray_control;

  struct jpeg_memory_mgr {
    alloc_small: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                           sizeofobject: size_t) -> *c_void,

    alloc_large: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                           sizeofobject: size_t) -> *c_void,

    alloc_sarray: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                            samplesperrow: JDIMENSION,
                            numrows: JDIMENSION) -> JSAMPARRAY,
    alloc_barray: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                            blocksperrow: JDIMENSION, numrows: JDIMENSION)
                            -> JBLOCKARRAY,

    request_virt_sarray: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                   pre_zero: boolean, samplesperrow: JDIMENSION,
                                   numrows: JDIMENSION, maxaccess: JDIMENSION)
                                   -> jvirt_sarray_ptr,

    request_virt_barray: extern fn(cinfo: j_common_ptr, pool_id: c_int,
                                   pre_zero: boolean, blocksperrow: JDIMENSION,
                                   numrows: JDIMENSION, maxaccess: JDIMENSION)
                                   -> jvirt_barray_ptr,

    realize_virt_arrays: extern fn(cinfo: j_common_ptr),

    access_virt_sarray: extern fn(cinfo: j_common_ptr, ptr: jvirt_sarray_ptr,
                                  start_row: JDIMENSION, num_rows: JDIMENSION,
                                  writable: boolean) -> JSAMPARRAY,

    access_virt_barray: extern fn(cinfo: j_common_ptr, ptr: jvirt_barray_ptr,
                                  start_row: JDIMENSION, num_rows: JDIMENSION,
                                  writable: boolean) -> JBLOCKARRAY,

    free_pool: extern fn(cinfo: j_common_ptr, pool_id: c_int),
    self_destruct: extern fn(cinfo: j_common_ptr),

    max_memory_to_use: c_long,
    max_alloc_chunk: c_long
  }

  // type jpeg_marker_parser_method =
  //   extern fn(cinfo: j_decompress_ptr) -> boolean;

  struct jvirt_sarray_control { dummy: c_long }
  struct jvirt_barray_control { dummy: c_long }
  struct jpeg_comp_master { dummy: c_long }
  struct jpeg_c_main_controller { dummy: c_long }
  struct jpeg_c_prep_controller { dummy: c_long }
  struct jpeg_c_coef_controller { dummy: c_long }
  struct jpeg_marker_writer { dummy: c_long }
  struct jpeg_color_converter { dummy: c_long }
  struct jpeg_downsampler { dummy: c_long }
  struct jpeg_forward_dct { dummy: c_long }
  struct jpeg_entropy_encoder { dummy: c_long }

  #[link(name = "jpeglib_macrofuns")]
  extern {
    fn jpeg_create_compress(cinfo: j_compress_ptr);
    // fn jpeg_create_decompress(cinfo: j_decompress_ptr);
  }

  #[link(name = "jpeg")]
  extern {
    fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: c_int,
                           structsize: size_t);
    // fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: c_int,
    //                          structsize: size_t);

    fn jpeg_abort_compress(cinfo: j_compress_ptr);

    fn jpeg_add_quant_table(cinfo: j_compress_ptr, which_tbl: c_int,
                            basic_table: *c_uint, scale_factor: c_int,
                            force_baseline: boolean);

    fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *JHUFF_TBL;
    fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *JQUANT_TBL;

    fn jpeg_default_colorspace(cinfo: j_compress_ptr);

    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    //fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);

    fn jpeg_quality_scaling(quality: c_int) -> c_int;

    fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);
    fn jpeg_set_defaults(cinfo: j_compress_ptr);
    fn jpeg_set_linear_quality (cinfo: j_compress_ptr, scale_factor: c_int,
                                force_baseline: boolean);
    fn jpeg_set_quality(cinfo: j_compress_ptr, quality: c_int,
                        force_baseline: boolean);

    fn jpeg_simple_progression(cinfo: j_compress_ptr);

    fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *FILE);
    // fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *FILE);

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

    // fn jpeg_read_header (cinfo: j_decompress_ptr, require_image: boolean)
    //                      -> c_int;

    // fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    // fn jpeg_read_scanlines(cinfo: j_decompress_ptr, scanlines: JSAMPARRAY,
    //                        max_lines: JDIMENSION) -> JDIMENSION;
    // fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    // fn jpeg_read_raw_data(cinfo: j_decompress_ptr, data: JSAMPIMAGE,
    //                       max_lines: JDIMENSION) -> JDIMENSION;

    // fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr) -> boolean;
    // fn jpeg_start_output(cinfo: j_decompress_ptr, scan_number: c_int)
    //                      -> boolean;
    // fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean;
    // fn jpeg_input_complete(cinfo: j_decompress_ptr) -> boolean;
    // fn jpeg_new_colormap(cinfo: j_decompress_ptr);
    // jpeg_consume_input(j_decompress_ptr cinfo) -> c_int;

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

}
