build:
	cargo build --release --manifest-path build-md/Cargo.toml
	cargo run --release --manifest-path build-md/Cargo.toml -- content/recipes
	zola build

clean:
	cargo clean --manifest-path build-md/Cargo.toml
	rm -rf public