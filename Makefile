# Typely Build System
# ==================

# Colors for pretty output
CYAN = \033[0;36m
GREEN = \033[0;32m
YELLOW = \033[1;33m
RED = \033[0;31m
NC = \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

# Variables
BACKEND_DIR = backend
CLI_DIR = clients/cli
GUI_DIR = clients/gui
FRONTEND_DIR = clients/frontend
BUILD_DIR = dist
TARGET_DIR = target

# Rust build profiles
PROFILE ?= release
FEATURES ?= system-integration

.PHONY: help clean install-deps install-rust check test all backend cli gui executable

## Display this help message
help:
	@echo "$(CYAN)Typely Build System$(NC)"
	@echo "=================="
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@grep -E '^## .*$$' $(MAKEFILE_LIST) | sed 's/## /  /'
	@echo ""
	@echo "$(YELLOW)Usage examples:$(NC)"
	@echo "  make all              # Build everything from scratch (includes dependencies)"
	@echo "  make setup            # Install Rust + system dependencies only"
	@echo "  make quick            # Build backend + CLI only (faster)"
	@echo "  make backend          # Build backend only"
	@echo "  make cli              # Build CLI only"
	@echo "  make gui              # Build GUI only"
	@echo "  make executable       # Build executable binaries"
	@echo "  make install          # Install to system"

## Install system dependencies
install-deps:
	@echo "$(CYAN)Installing system dependencies...$(NC)"
	@if command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get update && sudo apt-get install -y \
			build-essential \
			libssl-dev \
			libsqlite3-dev \
			pkg-config \
			libx11-dev \
			libxi-dev \
			libxtst-dev \
			libgtk-3-dev \
			libwebkit2gtk-4.1-dev \
			librsvg2-dev; \
	elif command -v dnf >/dev/null 2>&1; then \
		sudo dnf install -y \
			gcc \
			openssl-devel \
			sqlite-devel \
			pkgconfig \
			libX11-devel \
			libXi-devel \
			libXtst-devel \
			gtk3-devel \
			webkit2gtk3-devel \
			librsvg2-devel; \
	elif command -v brew >/dev/null 2>&1; then \
		brew install sqlite; \
	else \
		echo "$(RED)Please install dependencies manually$(NC)"; \
	fi
	@echo "$(GREEN)Dependencies installed!$(NC)"

## Install Rust toolchain
install-rust:
	@echo "$(CYAN)Installing Rust toolchain...$(NC)"
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "$(YELLOW)Rust not found, installing...$(NC)"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		echo "$(YELLOW)Please restart your shell or run: source $$HOME/.cargo/env$(NC)"; \
		source $$HOME/.cargo/env 2>/dev/null || true; \
		export PATH="$$HOME/.cargo/bin:$$PATH"; \
	else \
		echo "$(GREEN)Rust already installed: $$(rustc --version)$(NC)"; \
	fi
	@echo "$(CYAN)Installing Tauri CLI...$(NC)"
	@source $$HOME/.cargo/env 2>/dev/null || true; \
	export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if ! command -v cargo-tauri >/dev/null 2>&1; then \
		cargo install tauri-cli --locked; \
	else \
		echo "$(GREEN)Tauri CLI already installed$(NC)"; \
	fi
	@echo "$(GREEN)Rust toolchain ready!$(NC)"

## Check code quality and formatting
check:
	@echo "$(CYAN)Running code checks...$(NC)"
	cd $(BACKEND_DIR) && cargo fmt --check
	cd $(BACKEND_DIR) && cargo clippy -- -D warnings
	cd $(GUI_DIR) && cargo fmt --check
	cd $(GUI_DIR) && cargo clippy -- -D warnings
	@echo "$(GREEN)All checks passed!$(NC)"

## Format code
fmt:
	@echo "$(CYAN)Formatting code...$(NC)"
	cd $(BACKEND_DIR) && cargo fmt
	cd $(GUI_DIR) && cargo fmt
	@echo "$(GREEN)Code formatted!$(NC)"

## Run tests
test:
	@echo "$(CYAN)Running tests...$(NC)"
	cd $(BACKEND_DIR) && cargo test --features $(FEATURES)
	@echo "$(GREEN)Tests passed!$(NC)"

## Build backend (CLI + Engine)
backend:
	@echo "$(CYAN)Building backend...$(NC)"
	cd $(BACKEND_DIR) && cargo build --profile $(PROFILE) --features $(FEATURES)
	@echo "$(GREEN)Backend built!$(NC)"

## Build GUI client
gui:
	@echo "$(CYAN)Building GUI client...$(NC)"
	cd $(GUI_DIR) && cargo tauri build
	@echo "$(GREEN)GUI client built!$(NC)"

## Build executables only
executable:
	@echo "$(CYAN)Building executables...$(NC)"
	cd $(BACKEND_DIR) && cargo build --profile $(PROFILE) --features $(FEATURES) --bins
	cd $(CLI_DIR) && cargo build --profile $(PROFILE)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(PROFILE)/typely $(BUILD_DIR)/
	@cp $(TARGET_DIR)/$(PROFILE)/typely-cli $(BUILD_DIR)/
	@chmod +x $(BUILD_DIR)/typely $(BUILD_DIR)/typely-cli
	@echo "$(GREEN)Executables built in $(BUILD_DIR)/$(NC)"
	@echo "  $(YELLOW)$(BUILD_DIR)/typely$(NC)     - Desktop application"
	@echo "  $(YELLOW)$(BUILD_DIR)/typely-cli$(NC) - Command-line tool"

## Build CLI only (no GUI dependencies)
cli:
	@echo "$(CYAN)Building CLI only...$(NC)"
	cd $(CLI_DIR) && cargo build --profile $(PROFILE)
	@mkdir -p $(BUILD_DIR)
	@cp $(TARGET_DIR)/$(PROFILE)/typely-cli $(BUILD_DIR)/
	@chmod +x $(BUILD_DIR)/typely-cli
	@echo "$(GREEN)CLI built in $(BUILD_DIR)/typely-cli$(NC)"

## Build everything from scratch (installs all dependencies)
all: install-rust install-deps backend cli gui
	@echo "$(GREEN)✓ All components built successfully!$(NC)"
	@echo "$(YELLOW)Available executables:$(NC)"
	@echo "  $(TARGET_DIR)/$(PROFILE)/typely     - Desktop application"
	@echo "  $(TARGET_DIR)/$(PROFILE)/typely-cli - Command-line tool"

## Create distribution packages
dist: all
	@echo "$(CYAN)Creating distribution packages...$(NC)"
	@mkdir -p $(BUILD_DIR)/packages
	
	# Copy executables
	@cp $(BACKEND_DIR)/target/$(PROFILE)/typely $(BUILD_DIR)/packages/
	@cp $(BACKEND_DIR)/target/$(PROFILE)/typely-cli $(BUILD_DIR)/packages/
	
	# Copy GUI bundles if they exist
	@if [ -d "$(GUI_DIR)/src-tauri/target/$(PROFILE)/bundle" ]; then \
		cp -r $(GUI_DIR)/src-tauri/target/$(PROFILE)/bundle/* $(BUILD_DIR)/packages/; \
	fi
	
	# Copy scripts and documentation
	@cp -r $(BACKEND_DIR)/scripts $(BUILD_DIR)/packages/
	@cp README.md $(BUILD_DIR)/packages/
	@cp -r $(FRONTEND_DIR) $(BUILD_DIR)/packages/docs
	
	@echo "$(GREEN)Distribution packages created in $(BUILD_DIR)/packages/$(NC)"

## Install to system
install: executable
	@echo "$(CYAN)Installing Typely to system...$(NC)"
	@sudo cp $(BUILD_DIR)/typely /usr/local/bin/
	@sudo cp $(BUILD_DIR)/typely-cli /usr/local/bin/
	@sudo chmod +x /usr/local/bin/typely /usr/local/bin/typely-cli
	@echo "$(GREEN)Typely installed to /usr/local/bin/$(NC)"

## Uninstall from system
uninstall:
	@echo "$(CYAN)Uninstalling Typely...$(NC)"
	@if [ -f "$(BACKEND_DIR)/scripts/uninstall.sh" ]; then \
		bash $(BACKEND_DIR)/scripts/uninstall.sh; \
	else \
		sudo rm -f /usr/local/bin/typely /usr/local/bin/typely-cli; \
	fi
	@echo "$(GREEN)Typely uninstalled!$(NC)"

## Clean build artifacts
clean:
	@echo "$(CYAN)Cleaning build artifacts...$(NC)"
	cd $(BACKEND_DIR) && cargo clean
	cd $(GUI_DIR) && cargo clean
	rm -rf $(BUILD_DIR)
	rm -rf $(TARGET_DIR)
	@echo "$(GREEN)Cleaned!$(NC)"

## Start development server
dev:
	@echo "$(CYAN)Starting development server...$(NC)"
	cd $(GUI_DIR) && cargo tauri dev

## Run CLI in development
dev-cli:
	@echo "$(CYAN)Running CLI in development...$(NC)"
	cd $(CLI_DIR) && cargo run -- $(ARGS)

## Run desktop app in development
dev-app:
	@echo "$(CYAN)Running desktop app in development...$(NC)"
	cd $(BACKEND_DIR) && cargo run --bin typely --features $(FEATURES) $(ARGS)

## Show project structure
tree:
	@echo "$(CYAN)Project Structure:$(NC)"
	@tree -I 'target|node_modules|dist' . || find . -type d -name target -prune -o -type d -name node_modules -prune -o -type d -name dist -prune -o -print | sed 's/[^/]*\//  /g'

## Show build status and info
status:
	@echo "$(CYAN)Build Status$(NC)"
	@echo "============"
	@echo "$(YELLOW)Rust Version:$(NC) $$(rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "$(YELLOW)Cargo Version:$(NC) $$(cargo --version 2>/dev/null || echo 'Not installed')"
	@echo "$(YELLOW)Profile:$(NC) $(PROFILE)"
	@echo "$(YELLOW)Features:$(NC) $(FEATURES)"
	@echo ""
	@echo "$(YELLOW)Build Artifacts:$(NC)"
	@if [ -f "$(TARGET_DIR)/$(PROFILE)/typely" ]; then \
		echo "  ✓ Backend (typely)"; \
	else \
		echo "  ✗ Backend (typely)"; \
	fi
	@if [ -f "$(TARGET_DIR)/$(PROFILE)/typely-cli" ]; then \
		echo "  ✓ CLI (typely-cli)"; \
	else \
		echo "  ✗ CLI (typely-cli)"; \
	fi
	@if [ -d "$(GUI_DIR)/src-tauri/target/$(PROFILE)" ]; then \
		echo "  ✓ GUI bundle"; \
	else \
		echo "  ✗ GUI bundle"; \
	fi

## Quick build (backend + CLI only, no GUI dependencies)
quick: install-rust install-deps backend cli
	@echo "$(GREEN)Quick build completed!$(NC)"
	@echo "$(YELLOW)Available executables:$(NC)"
	@echo "  $(TARGET_DIR)/$(PROFILE)/typely     - Desktop application"
	@echo "  $(TARGET_DIR)/$(PROFILE)/typely-cli - Command-line tool"
	@echo "$(YELLOW)Tip: Run 'make gui' to build the GUI client$(NC)"

## Quick build and test
test-quick: quick test
	@echo "$(GREEN)Quick build and test completed!$(NC)"

## Release build (optimized)
release:
	$(MAKE) all PROFILE=release FEATURES=system-integration

## Debug build
debug:
	$(MAKE) all PROFILE=debug FEATURES=system-integration

# Special targets for CI/CD
.PHONY: ci-test ci-build ci-lint
ci-test: install-rust install-deps test
ci-build: install-rust install-deps backend cli gui
ci-lint: install-rust check

## One-command setup for new machines
setup: install-rust install-deps
	@echo "$(GREEN)✓ Setup complete!$(NC)"
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  make all     # Build everything (including GUI)"
	@echo "  make quick   # Build backend + CLI only (faster)"
	@echo "  make install # Build and install to system"