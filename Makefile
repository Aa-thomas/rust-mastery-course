SHELL := /bin/bash

.PHONY: setup fmt clippy test ci build bench doc run hook-install demo burst modes

setup: hook-install

fmt: ; cargo fmt --all -- --check
clippy: ; cargo clippy --workspace --all-targets || true
test: ; cargo test --workspace --all-features
ci: fmt clippy test
build: ; cargo build --workspace --all-targets
bench: ; cargo bench --workspace || true
doc: ; cargo doc --workspace --no-deps

run:
	@if [ -z "$(LAB)" ]; then echo "Usage: make run LAB=path/to/crate"; exit 1; fi
	@cd $(LAB) && cargo run

hook-install:
	@if [ ! -d .git ]; then echo "No .git dir. Run 'git init' first."; exit 1; fi
	@install -m 0755 scripts/pre-commit.sh .git/hooks/pre-commit && echo "pre-commit installed"

demo: ; cargo run -p capstone_axum_gateway
burst: ; bash scripts/burst.sh
modes: ; bash scripts/modes.sh
