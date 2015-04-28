all: libstegpeg

rebuild: clean all

lib:
	mkdir -p lib

libstegpeg: src/libstegpeg/lib.rs lib/libjpeglib_macrofuns.a
	rustc -g -L ./lib --out-dir=lib $<

lib/libjpeglib_macrofuns.a: c_src/jpeglib_macrofuns.o lib
	$(AR) rcs $@ $<

tests/test: tests/main.rs libstegpeg lib/libjpeglib_macrofuns.a
	rustc -g -L ./lib -l stegpeg --test -o $@ $<

tests/lsb: tests/lsb.rs libstegpeg lib/libjpeglib_macrofuns.a
	rustc -g -L ./lib -l stegpeg --test -o $@ $<

%.o: %.c
	$(CC) -fPIC $< -c -o $@

clean:
	rm -rf lib tests/test c_src/*.o mtest

test: tests/test tests/lsb
	# LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" $^
	$(foreach test,$^, LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" $(test);)

# Use for manual testing
mtest:
	rustc -L ./lib -l stegpeg -g -o mtest test.rs
	cat tests/files/test.txt | LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest tests/files/lena.jpg /tmp/out.jpg
	LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest /tmp/out.jpg

.PHONY: all rebuild clean test libstegpeg mtest
