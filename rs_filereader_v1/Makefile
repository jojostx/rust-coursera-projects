rust-versions:
	@echo "Rust command-line utilities versions:"
	rustc --version						# rust compiler
	cargo --version						# rust package manager
	rustfmt --version					# rust formatter
	clippy-driver --version		# rust linter
	rustup --version					# rust toolchain manager

test:
	cargo test

run:
	cargo run --quiet

lint:
	cargo clippy --quiet

format:
	cargo fmt --quiet

release:
	cargo build --release

clean:
	cargo clean

all:
	clean lint format test release