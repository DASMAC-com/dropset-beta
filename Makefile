# cspell:word vite
.PHONY: all
.PHONY: asm
.PHONY: clean
.PHONY: test

all: docs-prettier pre-commit-lint
clean:
test:

# Assemble the program.
asm:
	cd program && sbpf build --arch v3 --deploy-dir deploy

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
		&& npx vitepress preview --open

# Run pre-commit lint checks on all files.
pre-commit-lint:
	pre-commit run --config cfg/pre-commit/lint.yml --all-files
