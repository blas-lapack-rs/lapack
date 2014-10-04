archive := lapack-3.5.0.tgz
url := http://www.netlib.org/lapack/$(archive)
library := $(build)/liblapack.a

$(library): src/librefblas.a src/liblapack.a
	mkdir -p build
	cd build && ar x ../src/librefblas.a
	cd build && ar x ../src/liblapack.a
	rm -f $@
	ar -r $@ build/*.o

src/librefblas.a: src/make.inc
	$(MAKE) -C src blaslib

src/liblapack.a: src/make.inc
	$(MAKE) -C src lapacklib

src/make.inc: $(archive)
	mkdir -p src
	tar -xzf $(archive) --strip=1 -C src
	cp $@.example $@

$(archive):
	curl $(url) -o $@

clean:
	rm -f $(library)

.PHONY: clean
