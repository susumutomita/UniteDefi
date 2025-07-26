.PHONY: install lint lint_fix test format_check setup_husky before_commit help

PNPM_RUN_TARGETS = preview

$(PNPM_RUN_TARGETS):
	pnpm run $@

.PHONY: lint
lint:
	pnpm run lint || true

.PHONY: install
install:
	pnpm install
	cargo --version || (echo "Rust is not installed. Please install from https://rustup.rs/" && exit 1)

.PHONY: lint_fix
lint_fix:
	pnpm run lint:fix

.PHONY: test
test:
	cargo test --workspace

.PHONY: format_check
format_check:
	cargo fmt --all -- --check

setup_husky:
	pnpm run husky

.PHONY: before_commit
before_commit: lint format_check test

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make install      - Install Node.js dependencies and check Rust"
	@echo "  make lint         - Run textlint on markdown files"
	@echo "  make lint_fix     - Fix textlint errors"
	@echo "  make test         - Run Rust tests"
	@echo "  make format_check - Check Rust code formatting"
	@echo "  make before_commit- Run all checks before commit"
