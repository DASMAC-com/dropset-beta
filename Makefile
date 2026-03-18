# cspell:word vite
.PHONY: all
.PHONY: asm
.PHONY: asm-test
.PHONY: bench
.PHONY: clean
.PHONY: test

all: docs-prettier pre-commit-lint
clean:

# Run test cases (requires `make asm-test` first).
test:
	cd tests && DROPSET_ASM_DIR=../target/asm-test RUST_LOG=none cargo test -- --nocapture

# Run CU benchmark report (requires `make asm-test` first).
bench:
	cd tests && DROPSET_ASM_DIR=../target/asm-test RUST_LOG=none cargo run --bin bench

# Assemble the program (runs build.rs injection first).
asm:
	cargo check
	cd program && sbpf build --arch v3 --deploy-dir ../target/asm

# Assemble the program without --arch v3 for Mollusk testing.
asm-test:
	cargo check
	cd program && sbpf build --deploy-dir ../target/asm-test

# Build and serve docs locally for development.
docs-dev:
	cd docs && npm install \
		&& rm -rf .vitepress/cache .vitepress/dist node_modules/.vite \
		&& npx vitepress dev --open

# Format docs with Prettier.
docs-prettier:
	cd docs && npm install && npx prettier --write .

# Build and serve docs locally in production mode.
docs-prod:
	cd docs \
		&& rm -rf .vitepress/cache .vitepress/dist node_modules/.vite \
		&& npm ci \
		&& npx vitepress build \
		&& (sleep 1 && open http://localhost:4173 &) && npx vitepress preview

# Run pre-commit lint checks on all files.
pre-commit-lint:
	pre-commit run --config cfg/pre-commit/lint.yml --all-files
