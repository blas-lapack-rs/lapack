flags := $(flags) -L $(build)
build := $(build)/test

src := lib.rs $(base)/src/lib.rs
bin := $(build)/lapack

all: $(build) $(bin)
	$(bin)

bench: $(build) $(bin)
	$(bin) --bench

$(bin): $(src)
	$(rustc) $< $(flags) --out-dir $(build) --test

$(build):
	mkdir -p $(build)

clean:
	rm -f $(bin)
	rm -fd $(build)

.PHONY: all bench clean
