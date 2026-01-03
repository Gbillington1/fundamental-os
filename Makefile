# Makefile for Fundamental OS

TARGET = aarch64-unknown-none
BINARY = target/$(TARGET)/debug/fundamental-os
IMG = kernel.img
RUST = src/main.rs src/exceptions.rs src/syscall.rs src/gic.rs src/timer.rs
ASM = src/boot.s src/exceptions.s

all: $(IMG)

$(IMG): $(RUST) $(ASM) linker.ld
	cargo build
	rust-objcopy -O binary $(BINARY) $(IMG)

run: $(IMG)
	qemu-system-aarch64 -M virt -cpu cortex-a57 -display none -serial stdio \
		-kernel $(IMG) 

# runs kernel in gdb
gdb: $(IMG)
	qemu-system-aarch64 -M virt -cpu cortex-a57 -display none \
		-serial stdio \
		-kernel $(IMG) -S -s & \
	\
	sleep 0.2; \
	\
	gdb -ex "target remote :1234" $(BINARY); \
	\
	kill $$(jobs -p)

clean:
	cargo clean
	rm -f $(IMG)
