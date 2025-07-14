.PHONY: clean-build docs lint build build-web hot run run-web

.ONESHELL: # Use one shell per target
	SHELL := /bin/bash
	# Stop excecution on any error
	.SHELLFLAGS = -ec

clean-build:
	cargo clean && make build && make lint

docs:
	cargo doc --open --no-deps --workspace

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	cargo machete

build:
	bevy build --locked --release

build-web:
	cargo binstall --locked -y --force wasm-bindgen-cli
	cargo binstall --locked -y --force wasm-opt
	bevy build --locked --release --features=web --yes web --bundle

hot:
	BEVY_ASSET_ROOT="." dx serve --hot-patch

run:
	cargo run

run-web:
	bevy run web --headers="Cross-Origin-Opener-Policy:same-origin" --headers="Cross-Origin-Embedder-Policy:credentialless"
