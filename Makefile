build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

ALLOWED_CLIPPY_LINTS:= \
	-A clippy::many_single_char_names \
	-A clippy::needless_range_loop \
	-A clippy::comparison_chain \
	-A clippy::ptr_arg

CLIPPY_FLAG=--workspace --all-targets --tests

clippy:
	cargo +nightly clippy ${CLIPPY_FLAG} --fix --allow-dirty --allow-staged -Z unstable-options -- ${ALLOWED_CLIPPY_LINTS}
	cargo +nightly clippy ${CLIPPY_FLAG} --no-default-features -- ${ALLOWED_CLIPPY_LINTS}

clean:
	cargo clean
