.PHONY: all
.PHONY: asm
.PHONY: bench
.PHONY: clean
.PHONY: test

SBPF_ARCH ?= v0
DEPLOY_DIR ?= target/asm

all: docs-prettier pre-commit-lint
clean:

# Run test cases.
test: asm
	cd tests && DROPSET_DEPLOY_DIR=../$(DEPLOY_DIR) RUST_LOG=none cargo test -- --nocapture

# Run CU benchmark report.
bench: asm
	cd tests && DROPSET_DEPLOY_DIR=../$(DEPLOY_DIR) RUST_LOG=none cargo run --bin bench

# Assemble the program (runs build.rs injection first).
asm:
	cargo check
	cd program && sbpf build --arch $(SBPF_ARCH) --deploy-dir ../$(DEPLOY_DIR)

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
