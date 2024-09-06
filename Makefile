build:
	cargo build --release
debug:
	cargo build
clean:
	cargo clean
lint:
	cargo clippy --all-targets --all-features -- -D warnings