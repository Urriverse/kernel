PROFILE ?= debug
KERNEL_PROFILE ?= $(PROFILE)
BUILD_PATH ?= target
RUSTFLAGS += -C link-arg=-Tetc/linker.ld -C relocation-model=static -C code-model=kernel -C no-redzone=y
ARTEFACT_PATH ?= /tmp/uyfgyuhijghfhjkl

build:
	@CARGO_TARGET_DIR="$(BUILD_PATH)" RUSTFLAGS="$(RUSTFLAGS)" cargo build $(KERNEL_CARGO_ARGS) --target x86_64-unknown-none
	@if [ ! -z "$(ARTEFACT_PATH)" ]; then echo "$(BUILD_PATH)/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH); fi

buildv:
	@CARGO_TARGET_DIR="$(BUILD_PATH)" RUSTFLAGS="$(RUSTFLAGS)" cargo build $(KERNEL_CARGO_ARGS) --target x86_64-unknown-none --verbose
	@if [ ! -z "$(ARTEFACT_PATH)" ]; then echo "$(BUILD_PATH)/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH); fi
