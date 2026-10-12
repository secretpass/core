# ==============================================================================
# Packaging make file
# ==============================================================================

.PHONY: all

# Default target when running `make`
all: help

# Shared configuration
WASM_TARGET    ?= web


web-local:
	$(MAKE) -C ./web local

web-cloud:
	$(MAKE) -C ./web cloud

ui-install:
	$(MAKE) -C ./cli/ui install

ui-build:
	$(MAKE) -C ./cli/ui build

bundle: web-local ui-install ui-build

cli:
	@echo "==> Building cli app"
	cd cli && \
	cargo build --release

# ------------------------------------------------------------------------------
# Clean
# ------------------------------------------------------------------------------
clean:
	@echo "==> Cleaning generated WASM packages and target directory..."
	rm -rf web/pkg web/pkg-local web/pkg-sandbox
	cargo clean

# ------------------------------------------------------------------------------
# Help
# ------------------------------------------------------------------------------
help:
	@echo "Usage:"
	@echo "  make web-local      Build WASM package for cli local storage usage (web/pkg-local/)"
	@echo "  make web-cloud      Build WASM package for cloud service usage (web/pkg-cloud/)"
	@echo "  make clean    		 Remove all built WASM artifacts and cargo cache"