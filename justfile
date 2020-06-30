tests:
	cargo test --features "local"

tests-ci:
	cargo test

@bench:
	cargo bench

@lint:
	rustup component add clippy
	rustup component add rustfmt
	cargo clippy --features "local" -- -D warnings
	cargo clippy --tests --features "local"
	cargo fmt -- --check

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
