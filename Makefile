.PHONY: help build test lint fmt check clean release docker-build docker-run install

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development
build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test --all-features

test-doc: ## Run documentation tests
	cargo test --doc

bench: ## Run benchmarks
	cargo bench

# Code quality
lint: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

check: fmt-check lint test ## Run all checks (format, lint, test)

audit: ## Run security audit
	cargo audit

udeps: ## Check for unused dependencies
	cargo +nightly udeps --all-targets

# Maintenance
clean: ## Clean build artifacts
	cargo clean

update: ## Update dependencies
	cargo update

# Release
changelog: ## Generate changelog
	git cliff --output CHANGELOG.md

release-dry-run: ## Dry run release process
	git cliff --tag v0.1.0

# Docker
docker-build: ## Build Docker image
	docker build -t blogr:latest .

docker-run: ## Run Docker container
	docker run -p 3000:3000 blogr:latest

docker-dev: ## Start development environment with Docker Compose
	docker-compose up blogr-dev

docker-prod: ## Start production environment with Docker Compose
	docker-compose up blogr

# Installation
install: ## Install the CLI tool
	cargo install --path blogr-cli

install-dev: ## Install development dependencies
	rustup component add clippy rustfmt
	cargo install cargo-audit cargo-udeps git-cliff cargo-chef

# CI/CD helpers
ci-check: fmt-check lint test audit ## Run all CI checks locally

coverage: ## Generate test coverage report
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out html

# Project management
init-blog: ## Initialize a new blog project (for testing)
	./target/release/blogr init test-blog

serve: ## Start development server
	cargo run --bin blogr -- serve

# Git hooks
setup-hooks: ## Setup git hooks
	@echo "Setting up git hooks..."
	@echo '#!/bin/sh\nmake ci-check' > .git/hooks/pre-push
	@chmod +x .git/hooks/pre-push
	@echo "Git hooks installed successfully"
