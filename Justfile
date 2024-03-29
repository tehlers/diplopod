default: run

run:
    cargo run

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
