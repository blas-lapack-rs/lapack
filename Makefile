export base := $(shell pwd)
export build := $(base)/build

export rustc := rustc
export flags := $(RUST_FLAGS)

src: build vendor
	@$(MAKE) -C src

test: src
	@$(MAKE) -C src test
	@$(MAKE) -C test

bench: src
	@$(MAKE) -C test bench

vendor: build
	@$(MAKE) -C vendor

build:
	mkdir -p build

clean:
	@$(MAKE) -C src clean
	@$(MAKE) -C test clean
	@$(MAKE) -C vendor clean

.PHONY: src test vendor clean
