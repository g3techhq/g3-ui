set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

default:
    @just --list

setup:
    lefthook install

format:
    cargo fmt --all
    cargo fmt --manifest-path playground/Cargo.toml

format-check:
    cargo fmt --all -- --check
    cargo fmt --manifest-path playground/Cargo.toml -- --check

check:
    cargo check
    cargo check --features transitions
    cargo check --manifest-path playground/Cargo.toml

lint:
    cargo clippy --all-targets --no-deps
    cargo clippy --all-targets --features transitions --no-deps
    cargo clippy --manifest-path playground/Cargo.toml --all-targets --no-deps

lint-strict:
    cargo clippy --all-targets --no-deps -- -D warnings
    cargo clippy --all-targets --features transitions --no-deps -- -D warnings
    cargo clippy --manifest-path playground/Cargo.toml --all-targets --no-deps -- -D warnings

test:
    cargo nextest run
    cargo nextest run --features transitions
    cargo test --doc --features transitions
    cargo test --manifest-path playground/Cargo.toml

spell:
    typos

script-test:
    node --test playground/script-tests.cjs

browser-test:
    node playground/browser-tests.cjs

doc:
    cargo doc --no-deps --features transitions --config "build.rustdocflags='-D warnings'"

security:
    cargo deny check

pre-push: format-check check lint-strict test script-test spell

quality: pre-push

package-check:
    cargo package --allow-dirty

ci: quality doc security package-check

package:
    cargo package

record-transitions:
    node playground/record-transitions.cjs
