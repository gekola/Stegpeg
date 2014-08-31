all: libstegpeg

rebuild: clean all

lib:
	mkdir -p lib

libstegpeg: src/libstegpeg/lib.rs lib/libjpeglib_macrofuns.a
	rustc -g -L ./lib --out-dir=lib $<

lib/libjpeglib_macrofuns.a: c_src/jpeglib_macrofuns.o lib
	$(AR) rcs $@ $<

tests/test: tests/main.rs libstegpeg
	rustc -g -L ./lib --test -o $@ $<

%.o: %.c
	$(CC) $< -c -o $@


clean:
	rm -rf lib tests/test c_src/*.o

test: tests/test
	$<

.PHONY: all rebuild clean test libstegpeg
