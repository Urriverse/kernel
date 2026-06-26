KERNEL_PROFILE ?= debug
BUILD_PATH ?= target

build:
	@CARGO_TARGET_DIR="$(BUILD_PATH)/kernel" cargo build $(KERNEL_CARGO_ARGS) --target x86_64-unknown-none
	@echo "$(PWD)/target/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH)

test:
	@cargo test
