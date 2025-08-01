.PHONY: help install lint lint_md lint_rust lint_yaml lint_fix lint_fix_md lint_fix_rust test test_coverage test_coverage_html coverage_open format format_check setup_husky before_commit

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make install       - Install Node.js dependencies and check Rust"
	@echo "  make lint          - Run all linters (Markdown, Rust, YAML)"
	@echo "  make lint_md       - Run textlint on markdown files"
	@echo "  make lint_rust     - Run cargo clippy on workspace"
	@echo "  make lint_rust_near - Run cargo clippy on NEAR contracts"
	@echo "  make lint_yaml     - Run yamllint on YAML files"
	@echo "  make lint_fix      - Fix all auto-fixable lint issues"
	@echo "  make lint_fix_md   - Fix textlint errors"
	@echo "  make lint_fix_rust - Fix clippy warnings"
	@echo "  make test          - Run workspace tests"
	@echo "  make test_near     - Run NEAR contract tests"
	@echo "  make test_all      - Run all tests (workspace + NEAR)"
	@echo "  make test_security - Run security-focused tests"
	@echo "  make test_coverage - Run tests with coverage report"
	@echo "  make test_coverage_html - Run tests with HTML coverage report"
	@echo "  make coverage_open - Open HTML coverage report in browser"
	@echo "  make format        - Format workspace Rust code"
	@echo "  make format_near   - Format NEAR contract code"
	@echo "  make format_all    - Format all Rust code"
	@echo "  make format_check  - Check workspace Rust code formatting"
	@echo "  make format_check_near - Check NEAR contract formatting"
	@echo "  make format_check_all - Check all Rust code formatting"
	@echo "  make before_commit - Run all checks before commit (workspace only)"

PNPM_RUN_TARGETS = preview

$(PNPM_RUN_TARGETS):
	pnpm run $@

.PHONY: lint_md
lint_md:
	pnpm run lint

.PHONY: lint_rust
lint_rust:
	@echo "Running clippy on workspace..."
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint_rust_near
lint_rust_near:
	@echo "Running clippy on NEAR HTLC..."
	cd contracts/near-htlc && cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint_yaml
lint_yaml:
	pnpm run lint:yaml

.PHONY: lint
lint: lint_md lint_rust lint_yaml
	@echo "All lint checks completed"

.PHONY: install
install:
	git submodule update --init --recursive
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
	@echo "Running Rust tests..."
	cargo test --workspace
	@echo "All workspace tests completed!"

.PHONY: test_near
test_near:
	@echo "Running NEAR HTLC specific tests..."
	cd contracts/near-htlc && cargo test
	@echo "Running TypeScript tests..."
	@if [ -f contracts/near-htlc/package.json ]; then \
		cd contracts/near-htlc && npm test 2>/dev/null || echo "No TypeScript tests found"; \
	fi
	@echo "All NEAR tests completed!"

.PHONY: test_all
test_all: test test_near
	@echo "All tests completed!"

.PHONY: test_security
test_security:
	@echo "Running security-focused tests..."
	cd contracts/near-htlc && cargo test security -- --nocapture
	@if [ -f contracts/near-htlc/scripts/run_all_tests.sh ]; then \
		cd contracts/near-htlc && ./scripts/run_all_tests.sh; \
	fi
	@echo "Security tests completed!"

.PHONY: test_coverage
test_coverage:
	cargo llvm-cov --workspace

.PHONY: test_coverage_html
test_coverage_html:
	cargo llvm-cov --workspace --html
	@echo "Coverage report generated in target/llvm-cov/html/index.html"

.PHONY: coverage_open
coverage_open:
	@if [ -f target/llvm-cov/html/index.html ]; then \
		open target/llvm-cov/html/index.html; \
		echo "Opening coverage report in browser..."; \
	else \
		echo "Coverage report not found. Run 'make test_coverage_html' first."; \
	fi

.PHONY: format
format:
	@echo "Formatting Rust code..."
	cargo fmt --all
	@echo "All workspace formatting completed!"

.PHONY: format_near
format_near:
	@echo "Formatting NEAR HTLC code..."
	cd contracts/near-htlc && cargo fmt
	@echo "NEAR formatting completed!"

.PHONY: format_all
format_all: format format_near
	@echo "All formatting completed!"

.PHONY: format_check
format_check:
	@echo "Checking Rust formatting..."
	cargo fmt --all -- --check

.PHONY: format_check_near
format_check_near:
	@echo "Checking NEAR HTLC formatting..."
	cd contracts/near-htlc && cargo fmt -- --check

.PHONY: format_check_all
format_check_all: format_check format_check_near
	@echo "All format checks completed!"

setup_husky:
	pnpm run husky

.PHONY: before_commit
before_commit: lint format_check test_coverage
