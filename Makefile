all: libstegpeg

libstegpeg: src/libstegpeg/lib.rs lib/libjpeglib_macrofuns.a
	mkdir -p lib
	rustc -L=./lib --out-dir=lib $<

lib/libjpeglib_macrofuns.a: c_src/jpeglib_macrofuns.c
	$(CC) $< -c -o $<.o
	$(AR) rcs $@ $<.o

clean:
	rm -rf lib


.PHONY: libstegpeg
