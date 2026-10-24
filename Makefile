SHELL := /usr/bin/env
.SHELLFLAGS = bash -e -o pipefail -c
.DEFAULT_GOAL := help
.NOTPARALLEL:
.SILENT: # use set -v to print commands executed
.ONESHELL:

ifndef VERBOSE
MAKEFLAGS += --no-print-directory
endif

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

CARGO ?= cargo

# release (default) or debug
PROFILE ?= release
CARGO_FLAGS = $(if $(filter release,$(PROFILE)),--release,)
EXE ?= ./target/$(PROFILE)/lingua

##@ Setup

.PHONY: setup
setup: ## Install rustup (if missing) and the pinned toolchain
	if ! command -v rustup >/dev/null; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none; \
		. "$$HOME/.cargo/env"; \
	fi
	rustup show active-toolchain || rustup toolchain install

##@ Build

.PHONY: build
build: ## Build codebase
	$(CARGO) build --locked $(CARGO_FLAGS)

.PHONY: clean
clean: ## Clean build space
	$(CARGO) clean

.PHONY: run
run: build ## Build and run the application
	$(EXE)

.PHONY: test
test: ## Run tests
	$(CARGO) test --locked

##@ Dependencies

.PHONY: deps
deps: ## Download dependencies
	$(CARGO) fetch --locked

.PHONY: upgrade
upgrade: ## Upgrade dependencies
	$(CARGO) update

##@ Code Quality

.PHONY: check
check: ## Check code quality
	$(CARGO) clippy --locked --all-targets -- -D warnings

.PHONY: format
format: ## Format code
	$(CARGO) fmt --all

##@ Helpers

.PHONY: help
help: ## Display this help
	awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

env-%: ## Check if env var is defined
	if [ -z "$($*)" ]; then \
		echo "Error: Environment variable '$*' is not set."; \
		exit 1; \
	fi
