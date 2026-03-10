.PHONY: all
.PHONY: clean
.PHONY: test

all: docs-prettier pre-commit-lint
clean:
test:

RUN_DOCS = cd docs && npx
docs-dev:
	$(RUN_DOCS) vitepress dev
docs-prettier:
	$(RUN_DOCS) prettier --write .
pre-commit-lint:
	pre-commit run --config cfg/pre-commit/lint.yml --all-files
