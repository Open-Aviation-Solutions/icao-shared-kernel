# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

.DEFAULT_GOAL := help

ADDLICENSE_VERSION := v1.2.0
# Installed here, at this version, so that every machine checks with the same one.
ADDLICENSE := $(CURDIR)/.bin/addlicense-$(ADDLICENSE_VERSION)/addlicense

.PHONY: help dev build test lint fmt fmt-check license-check check

help: ## List available targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

dev: build ## Start the development environment (builds the crate)

build: ## Compile the crate
	cargo build

test: ## Run the test suite
	cargo test

lint: ## Lint with clippy, treating warnings as errors
	cargo clippy --all-targets -- -D warnings

fmt: ## Format the code
	cargo fmt

fmt-check: ## Check formatting without modifying files
	cargo fmt --check

$(ADDLICENSE):
	GOBIN=$(dir $(ADDLICENSE)) go install github.com/google/addlicense@$(ADDLICENSE_VERSION)

# addlicense checks the tracked files of the types it knows (Rust, TOML, YAML)
# for a licence header, and passes over the rest (Cargo.lock, Markdown, LICENSE).
# It does not recognise a Makefile, so this one is checked on its own.
license-check: $(ADDLICENSE) ## Check that each source file carries a licence header
	git ls-files -z | xargs -0 $(ADDLICENSE) -check -l mpl
	@head -3 Makefile | grep -qF 'file, You can obtain one at https://mozilla.org/MPL/2.0/.' \
		|| { echo "Makefile: the MPL 2.0 notice is missing from its first lines"; exit 1; }

check: lint fmt-check license-check test ## Run all checks: lint, format check, licence headers, tests
