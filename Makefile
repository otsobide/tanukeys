.DEFAULT_GOAL := build

APPS :=

# Per-app targets — delegated to apps/<name>/Makefile
define APP_RULES
.PHONY: $(1)/build $(1)/dev-build $(1)/run $(1)/test $(1)/test/e2e
$(1)/build:
	$$(MAKE) -C apps/$(1) build
$(1)/dev-build:
	$$(MAKE) -C apps/$(1) dev-build
$(1)/run:
	$$(MAKE) -C apps/$(1) run
$(1)/test:
	$$(MAKE) -C apps/$(1) test
$(1)/test/e2e:
	$$(MAKE) -C apps/$(1) test/e2e
endef
$(foreach app,$(APPS),$(eval $(call APP_RULES,$(app))))

# Global targets

.PHONY: build
build:
	cargo build --release

.PHONY: dev/build
dev/build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: test/unit
test/unit:
	cargo test --workspace

.PHONY: test/e2e
test/e2e: $(foreach app,$(APPS),$(app)/test/e2e)

.PHONY: format
format:
	cargo fmt

.PHONY: audit
audit:
	cargo install cargo-audit || echo "cargo-audit already installed"
	cargo audit

.PHONY: deps
deps:
	cargo update
