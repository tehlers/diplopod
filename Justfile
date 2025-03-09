default: run

run:
    cargo run

run-wasm:
    # Needs wasm-server-runner (install with `cargo install wasm-server-runner`)
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER='wasm-server-runner' cargo run --target wasm32-unknown-unknown

format:
    cargo fmt

check-format:
    cargo fmt --check

test:
    cargo test

lint:
    cargo clippy

check: check-format test lint

install:
    cargo install --path .

uninstall:
    cargo uninstall diplopod
