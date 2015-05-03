TESTS= $(patsubst tests/%.rs,%.test,$(wildcard tests/*.rs))
TEST_INPUT_SIZES= 10k 100k

all: libstegpeg

rebuild: clean all

lib:
	mkdir -p lib

libstegpeg: src/libstegpeg/lib.rs lib/libjpeglib_macrofuns.a
	rustc -g -L ./lib --out-dir=lib $<

lib/libjpeglib_macrofuns.a: c_src/jpeglib_macrofuns.o lib
	$(AR) rcs $@ $<

tests/%: tests/%.rs libstegpeg
	rustc -g -L ./lib -l stegpeg --test -o $@ $<

tests/files/test%k.txt:
	for i in {1..$*}; do cat tests/files/test1k.txt; done > tests/files/test$*k.txt

%.o: %.c
	$(CC) -fPIC $< -c -o $@

clean:
	rm -rf lib tests/test c_src/*.o mtest

%.test: tests/% tests-prepare
	@LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" $<

test: $(TESTS)

test-prepare: $(foreach sz,$(TEST_INPUT_SIZES),tests/files/test$(sz).txt)

# Use for manual testing
mtest:
	rustc -L ./lib -l stegpeg -g -o mtest test.rs
	cat tests/files/test.txt | LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest tests/files/lena.jpg /tmp/out.jpg
	LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest /tmp/out.jpg

.PHONY: all rebuild clean test libstegpeg tests-prepare mtest %.test
