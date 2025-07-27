.PHONY: help install lint lint_md lint_rust lint_yaml lint_fix lint_fix_md lint_fix_rust test test_coverage test_coverage_html format format_check setup_husky before_commit

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make install       - Install Node.js dependencies and check Rust"
	@echo "  make lint          - Run all linters (Markdown, Rust, YAML)"
	@echo "  make lint_md       - Run textlint on markdown files"
	@echo "  make lint_rust     - Run cargo clippy on Rust code"
	@echo "  make lint_yaml     - Run yamllint on YAML files"
	@echo "  make lint_fix      - Fix all auto-fixable lint issues"
	@echo "  make lint_fix_md   - Fix textlint errors"
	@echo "  make lint_fix_rust - Fix clippy warnings"
	@echo "  make test          - Run Rust tests"
	@echo "  make test_coverage - Run tests with coverage report"
	@echo "  make test_coverage_html - Run tests with HTML coverage report"
	@echo "  make format        - Format Rust code"
	@echo "  make format_check  - Check Rust code formatting"
	@echo "  make before_commit - Run all checks before commit"

PNPM_RUN_TARGETS = preview

$(PNPM_RUN_TARGETS):
	pnpm run $@

.PHONY: lint_md
lint_md:
	pnpm run lint

.PHONY: lint_rust
lint_rust:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint_yaml
lint_yaml:
	pnpm run lint:yaml

.PHONY: lint
lint: lint_md lint_rust lint_yaml
	@echo "All lint checks completed"

.PHONY: install
install:
	pnpm install
	cargo --version || (echo "Rust is not installed. Please install from https://rustup.rs/" && exit 1)

.PHONY: lint_fix_md
lint_fix_md:
	pnpm run lint:fix

.PHONY: lint_fix_rust
lint_fix_rust:
	cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

.PHONY: lint_fix
lint_fix: lint_fix_md lint_fix_rust
	@echo "All lint fixes completed"

.PHONY: test
test:
	cargo test --workspace

.PHONY: test_coverage
test_coverage:
	cargo llvm-cov --workspace

.PHONY: test_coverage_html
test_coverage_html:
	cargo llvm-cov --workspace --html
	@echo "Coverage report generated in target/llvm-cov/html/index.html"

.PHONY: format
format:
	cargo fmt --all

.PHONY: format_check
format_check:
	cargo fmt --all -- --check

setup_husky:
	pnpm run husky

.PHONY: before_commit
before_commit: lint format_check test_coverage
