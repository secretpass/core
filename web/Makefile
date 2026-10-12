# ==============================================================================
# Packaging make file
# ==============================================================================

.PHONY: all

# Default target when running `make`
all: help

# Shared configuration
WASM_TARGET    ?= web

# ------------------------------------------------------------------------------
# Library Local Build
# ------------------------------------------------------------------------------
local:
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.localhost" \
	WEBAUTH_RP_ORIGIN="http://secretpass.localhost" \
	WEBAUTH_RP_ENTITY="Secretpass Local" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	wasm-pack build \
		--target $(WASM_TARGET) \
		--release \
		--no-pack \
		--out-dir ../cli/ui/src/core


# ------------------------------------------------------------------------------
# Library Cloud Build
# ------------------------------------------------------------------------------
cloud:
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.cloud" \
	WEBAUTH_RP_ORIGIN="https://secretpass.cloud" \
	WEBAUTH_RP_ENTITY="Secretpass Cloud" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	wasm-pack build \
		--target $(WASM_TARGET) \
		--release \
		--scope secretpass


# ------------------------------------------------------------------------------
# Help
# ------------------------------------------------------------------------------
help:
	@echo "Usage:"
	@echo "  local      Build WASM package for cli local storage usage (web/pkg-local/)"
	@echo "  cloud      Build WASM package for cloud service usage (web/pkg-cloud/)"
