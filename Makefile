.PHONY: install lint lint_fix setup_husky before_commit help

PNPM_RUN_TARGETS = lint preview

$(PNPM_RUN_TARGETS):
	pnpm run $@

.PHONY: install
install:
	pnpm install

.PHONY: lint_fix
lint_fix:
	pnpm run lint:fix

setup_husky:
	pnpm run husky

.PHONY: before_commit
before_commit: lint
