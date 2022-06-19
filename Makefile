


build:
	cargo lambda build --release --target aarch64-unknown-linux-musl
	sam deploy
