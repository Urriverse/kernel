KERNEL_PROFILE ?= debug

build:
	@cargo build $(KERNEL_CARGO_ARGS)
	@echo "$(PWD)/target/x86_64-unknown-none/$(KERNEL_PROFILE)/kernel" > $(ARTEFACT_PATH)

test:
	@cargo test
