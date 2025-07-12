.PHONY: clean docs lint build build-web run run-web hot

.ONESHELL: # Use one shell per target
	SHELL := /bin/bash
	# Stop excecution on any error
	.SHELLFLAGS = -ec

clean:
	cargo clean && make build && make lint

docs:
	cargo docs --open

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
