#!/usr/bin/env just --justfile
alias r := run
alias rr := run-release
alias b := build
alias br := build-release
alias i := install
alias h := help
alias q := runq

# build debug binary and copy to ~/bin
build:
	cargo build

# build release binary and copy to ~/bin
build-release:
	cargo build --release

# build release binary and install to cargo bin dir
install:
	cargo install --path . -f

# build debug binary and run
run:
	cargo run

# build release binary and run
run-release:
	cargo run --release

# run with --quiet
runq:
	./target/release/sysinfo -q

help:
	./target/release/sysinfo -h

# run with verbosity (INFO)
runv:
	RUST_LOG=info cargo run

# run with verbosity (DEBUG)
runvv:
	RUST_LOG=debug cargo run

# run release binary
sysinfo args='':
	./target/release/sysinfo {{args}}

test:
	cargo test

fix:
	cargo fix