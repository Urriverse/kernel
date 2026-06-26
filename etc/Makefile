PROFILE ?= debug
KERNEL_PROFILE ?= $(PROFILE)
BUILD_PATH ?= target
RUSTFLAGS = "-C link-arg=-Tetc/linker.ld -C relocation-model=static -C code-model=kernel -C no-redzone=y"

build:
	@CARGO_TARGET_DIR="$(BUILD_PATH)/kernel" cargo build $(KERNEL_CARGO_ARGS) --target x86_64-unknown-none
	@echo "$(BUILD_PATH)/kernel/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH)

test:
	@cargo test
