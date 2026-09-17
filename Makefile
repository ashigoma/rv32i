CC      := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
OBJDUMP := riscv64-unknown-elf-objdump

CPU_DIR := ./rv32i
CPU     := $(CPU_DIR)/build/main

COMPILER_DIR := ./ml_compiler
COMPILER     := $(COMPILER_DIR)/target/debug/ml_compiler
BUILD_DIR    := ./build

BOOT_S        := ./boot.s
LINKER_SCRIPT := ./link.ld

CFLAGS  := -march=rv32i -mabi=ilp32 -O0 -nostdlib -ffreestanding -I$(BOOT_S)
LDFLAGS := -T $(LINKER_SCRIPT) -Wl,--no-warn-rwx-segments

VPATH := src:ml_compiler/tests

ifeq ($(firstword $(MAKECMDGOALS)),run)
  RUN_TARGET := $(word 2, $(MAKECMDGOALS))
  $(eval $(RUN_TARGET):;@:)
endif

.PHONY: all cpu compiler clean run
.SECONDARY:

all: cpu compiler

cpu:
	$(MAKE) -C $(CPU_DIR)

compiler:
	cd $(COMPILER_DIR) && cargo build

$(BUILD_DIR)/%.s: %.ml compiler
	@mkdir -p $(BUILD_DIR)
	$(COMPILER) -i $< -o $@

$(BUILD_DIR)/%.elf: $(BUILD_DIR)/%.s $(BOOT_S) $(LINKER_SCRIPT)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(BOOT_S) $< $(LDFLAGS) -o $@

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf
	$(OBJCOPY) -O binary $< $@
	$(OBJDUMP) -D $< > $(BUILD_DIR)/$*.dis

run: $(BUILD_DIR)/$(RUN_TARGET).bin cpu
	@mkdir -p $(BUILD_DIR)
	$(CPU) +EXEC=$(BUILD_DIR)/$(RUN_TARGET).bin \
	       +TRACE_FILE=$(BUILD_DIR)/$(RUN_TARGET).trace \
	       +LOG_FILE=$(BUILD_DIR)/$(RUN_TARGET).log \
	       +VCD_FILE=$(BUILD_DIR)/$(RUN_TARGET).vcd

%: $(BUILD_DIR)/%.bin
	@:

clean:
	rm -rf $(BUILD_DIR)
	cargo clean --manifest-path $(COMPILER_DIR)/Cargo.toml
	$(MAKE) -C $(CPU_DIR) clean