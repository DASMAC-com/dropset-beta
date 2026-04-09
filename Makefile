.PHONY: all
.PHONY: asm
.PHONY: docs
.PHONY: clean
.PHONY: lint
.PHONY: test

SBPF_ARCH ?= v0
DEPLOY_DIR ?= target/asm

all: lint test
clean:
	cargo clean
	rm -rf docs/node_modules docs/.vitepress/cache docs/.vitepress/dist
	rm -rf $(DEPLOY_DIR)

# Run test cases.
test: asm
	cd tests \
		&& RUSTFLAGS='-Dwarnings' RUST_LOG=none \
		DROPSET_DEPLOY_DIR=../$(DEPLOY_DIR) cargo test -- --nocapture

# Assemble the program (runs build.rs injection first).
asm:
	RUSTFLAGS='-Dwarnings' cargo check
	cd program && sbpf build --arch $(SBPF_ARCH) --deploy-dir ../$(DEPLOY_DIR)

# Build docs (clean install + VitePress production build).
docs-build:
	cd docs \
		&& rm -rf .vitepress/cache .vitepress/dist node_modules/.vite \
		&& npm ci \
		&& npx vitepress build

# Check docs for broken links and anchors.
docs-links: docs-build
	lychee --config cfg/lychee.toml --include-fragments \
		--root-dir docs/.vitepress/dist 'docs/.vitepress/dist/**/*.html'

# Build and serve docs locally for development.
docs:
	cd docs && npm install \
		&& rm -rf .vitepress/cache .vitepress/dist node_modules/.vite \
		&& npx vitepress dev --open

# Format docs with Prettier.
docs-prettier:
	cd docs && npm install && npx prettier --write .

# Build and serve docs locally in production mode.
docs-prod: docs-build
	cd docs && (sleep 1 && open http://localhost:4173 &) && npx vitepress preview

# Run all lint checks.
lint: pre-commit-lint docs-prettier

# Run pre-commit lint checks on all files.
pre-commit-lint:
	pre-commit run --config cfg/pre-commit-lint.yml --all-files
