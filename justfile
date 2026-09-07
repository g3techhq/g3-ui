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

spell:
    typos

security:
    cargo deny check

pre-push: format-check check lint test spell

quality: pre-push

ci: quality security

package:
    cargo package

record-transitions:
    node playground/record-transitions.cjs
