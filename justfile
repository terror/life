ci: build test clippy fmt-check

build:
	wasm-pack build

serve:
	cd pkg/www && npm run start

test:
	wasm-pack test --chrome --headless

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run *args:
	cargo run -- --{{args}}

fmt: fmt-pkg
	cargo +nightly fmt

fmt-pkg:
	cd pkg && prettier --write .

check:
 cargo check

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"
