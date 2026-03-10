.PHONY: all
.PHONY: clean
.PHONY: test

all: docs-prettier quick-lint
clean:
test:

RUN_DOCS = cd docs && npx
docs-dev:
	$(RUN_DOCS) vitepress dev
docs-prettier:
	$(RUN_DOCS) prettier --write .
quick-lint:
	pre-commit run --config cfg/pre-commit/quick-lint.yml --all-files
