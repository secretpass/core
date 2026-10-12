# ==============================================================================
# Packaging make file
# ==============================================================================

.PHONY: all

# Default target when running `make`
all: help


add-target:
	@echo "==> add wasm target"
	rustup target add wasm32-unknown-unknown

# ------------------------------------------------------------------------------
# Library Local Build
# ------------------------------------------------------------------------------
local:
	$(MAKE) add-target
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.localhost" \
	WEBAUTH_RP_ORIGIN="http://secretpass.localhost" \
	WEBAUTH_RP_ENTITY="Secretpass Local" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	cargo rustc -p web --release --crate-type cdylib --target wasm32-unknown-unknown
	wasm-bindgen \
      ../target/wasm32-unknown-unknown/release/web.wasm \
      --target web \
      --out-dir ../cli/ui/src/core



# ------------------------------------------------------------------------------
# Library Cloud Build
# ------------------------------------------------------------------------------
cloud:
	$(MAKE) add-target
	@echo "==> Building WASM for cli server usage"
	WEBAUTH_RP_ID="secretpass.cloud" \
	WEBAUTH_RP_ORIGIN="https://secretpass.cloud" \
	WEBAUTH_RP_ENTITY="Secretpass Cloud" \
	RUSTFLAGS='--cfg web_sys_unstable_apis' \
	cargo rustc -p web --release --crate-type cdylib --target wasm32-unknown-unknown
	wasm-bindgen \
	  ../target/wasm32-unknown-unknown/release/web.wasm \
	  --target web \
	  --out-dir ../target/dist


# ------------------------------------------------------------------------------
# Help
# ------------------------------------------------------------------------------
help:
	@echo "Usage:"
	@echo "  local      Build WASM package for cli local storage usage (web/pkg-local/)"
	@echo "  cloud      Build WASM package for cloud service usage (web/pkg-cloud/)"
