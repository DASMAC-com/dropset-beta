.PHONY: docs
RUN_DOCS = cd docs && npx
docs:
	$(RUN_DOCS) vitepress dev
docs-prettier:
	$(RUN_DOCS) prettier --write .
