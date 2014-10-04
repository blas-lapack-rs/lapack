flags := $(flags) -L $(build)/vendor

src := lib.rs
lib := $(build)/liblapack.rlib
bin := $(build)/lapack

all: $(lib)

test: $(bin)
	$(bin)

$(lib): $(src)
	$(rustc) $< $(flags) --out-dir $(build)

$(bin): $(src)
	$(rustc) $< $(flags) --out-dir $(build) --test

clean:
	rm -f $(lib)

.PHONY: all test clean
