.PHONY: all
.PHONY: asm
.PHONY: debugger
.PHONY: docs
.PHONY: clean
.PHONY: lint
.PHONY: test

DEPLOY_DIR ?= target/deploy
ANCHOR ?= anchor

all: lint test

check-anchor:
	@$(ANCHOR) --version 2>/dev/null | grep -q "2\." \
		|| (echo "error: anchor-cli 2.x required (got: $$($(ANCHOR) --version 2>/dev/null || echo 'not found'))" >&2; exit 1)

clean:
	cargo clean
	rm -rf docs/node_modules docs/.vitepress/cache docs/.vitepress/dist
	rm -rf $(DEPLOY_DIR)

# Assemble the program: inject .equ constants, then build via LLVM.
asm: check-anchor
	RUSTFLAGS='-Dwarnings' cargo check
	cd program && $(ANCHOR) build --no-idl

# Run test cases.
test: asm
	cd tests \
		&& RUSTFLAGS='-Dwarnings' RUST_LOG=none \
		DROPSET_DEPLOY_DIR=../program/target/deploy cargo test -- --nocapture

# Launch the anchor debugger TUI.
debugger: asm
	cd program && $(ANCHOR) debugger --skip-build

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
