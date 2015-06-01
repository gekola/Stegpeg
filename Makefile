TESTS = $(patsubst tests/%.rs,%.test,$(wildcard tests/*.rs))
TEST_INPUT_SIZES = 10k 40k 100k

test-prepare: $(foreach sz,$(TEST_INPUT_SIZES),tests/files/test$(sz).txt)

# Use for manual testing
mtest:
	rustc -L ./lib -l stegpeg -g -o mtest test.rs
	cat tests/files/test10k.txt | LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest tests/files/lena.jpg /tmp/out.jpg
	LD_LIBRARY_PATH="$LD_LIBRARY_PATH:./lib" ./mtest /tmp/out.jpg

.PHONY: tests-prepare mtest
