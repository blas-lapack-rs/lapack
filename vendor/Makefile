export build := $(build)/vendor

all: build lapack

lapack:
	@$(MAKE) -C lapack

build:
	mkdir -p $(build)

clean:
	@$(MAKE) -C lapack clean
	rm -fd $(build)

.PHONY: all lapack clean
