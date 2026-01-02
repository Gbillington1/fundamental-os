# Makefile for Fundamental OS

TARGET = aarch64-unknown-none
BINARY = target/$(TARGET)/debug/fundamental-os
IMG = kernel.img

all: $(IMG)

$(IMG): src/main.rs src/exceptions.rs src/syscall.rs src/boot.s src/exceptions.s linker.ld
	cargo build
	rust-objcopy -O binary $(BINARY) $(IMG)

run: $(IMG)
	qemu-system-aarch64 -M virt -cpu cortex-a57 -display none -serial stdio \
		-kernel $(IMG) 

clean:
	cargo clean
	rm -f $(IMG)
