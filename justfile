default:
    @just --list

build:
    cargo build

clean:
    cargo clean

unit:
    cargo test --lib

e2e:
    cargo test --test test_e2e

test:
    cargo test
