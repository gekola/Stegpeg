all: libstegpeg

libstegpeg: src/libstegpeg/lib.rs
	mkdir -p lib
	rustc --out-dir=lib $<

clean:
	rm -rf lib


.PHONY: libstegpeg
