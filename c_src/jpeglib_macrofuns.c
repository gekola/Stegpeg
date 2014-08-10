#include <stdio.h>
#include <jpeglib.h>

void jpeg_create_compress_fn(j_compress_ptr cinfo)
{
  jpeg_create_compress(cinfo);
}

void jpeg_create_decompress_fn(j_decompress_ptr cinfo)
{
  jpeg_create_decompress(cinfo);
}
