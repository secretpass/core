# ==============================================================================
# Packaging make file
# ==============================================================================

.PHONY: all dev sandbox clean help

# Default target when running `make`
all: help

# Shared configuration
WASM_TARGET    ?= web

# ------------------------------------------------------------------------------
# Web Library Local Build
# ------------------------------------------------------------------------------
web-local:
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.localhost" \
	WEBAUTH_RP_ORIGIN="http://secretpass.localhost" \
	WEBAUTH_RP_ENTITY="Secretpass Local" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	cd web && \
	wasm-pack build \
		--target $(WASM_TARGET) \
		--release \
		--no-pack \
		--out-dir ../cli/ui/src/core


# ------------------------------------------------------------------------------
# Web Library Cloud Build
# ------------------------------------------------------------------------------
web-cloud:
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.cloud" \
	WEBAUTH_RP_ORIGIN="https://secretpass.cloud" \
	WEBAUTH_RP_ENTITY="Secretpass Cloud" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	cd web && \
	wasm-pack build \
		--target $(WASM_TARGET) \
		--release \
		--scope secretpass


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