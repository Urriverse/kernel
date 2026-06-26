PROFILE ?= debug
KERNEL_PROFILE ?= $(PROFILE)
BUILD_PATH ?= target

build:
	@echo "$$$$$$ PROFILE=$(PROFILE)"
	@echo "$$$$$$ KERNEL_PROFILE=$(KERNEL_PROFILE)"
	@echo "$$$$$$ BUILD_PATH=$(BUILD_PATH)"
	@echo "$$$$$$ KERNEL_CARGO_ARGS=$(KERNEL_CARGO_ARGS)"
	@echo "$$$$$$ ARTEFACT_PATH=$(ARTEFACT_PATH)"
	@CARGO_TARGET_DIR="$(BUILD_PATH)/kernel" cargo build $(KERNEL_CARGO_ARGS) --target x86_64-unknown-none
	@echo "$(PWD)/target/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH)

test:
	@cargo test
