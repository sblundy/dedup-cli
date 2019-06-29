.PHONY: all release clean test

all: target/x86_64-apple-darwin/release/dedup target/x86_64-apple-darwin/release/dedup
release: test target/dedup-cli.x86_64-apple-darwin.tgz target/dedup-cli.x86_64-unknown-linux-gnu.tgz
clean:
	cargo clean
test:
	cargo test

target/x86_64-apple-darwin/release/dedup:
	cargo build --target x86_64-apple-darwin --release

target/x86_64-unknown-linux-gnu/release/dedup:
	docker  run \
			--rm \
			--user $$(id -u):$$(id -g) \
			-v $(PWD):/usr/src/myapp \
			-w /usr/src/myapp \
			rust:1.35.0 \
			cargo build --target x86_64-unknown-linux-gnu --release

target/dedup-cli.%.tgz: target/%/release/dedup
	tar -c -z \
		-C target/$*/release/ \
		-f $@ \
		dedup