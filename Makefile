CC      := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
OBJDUMP := riscv64-unknown-elf-objdump

CPU_DIR := ./rv32i
CPU     := $(CPU_DIR)/build/main
TEST_SH := $(CPU_DIR)/src/test/scripts/test.sh

COMPILER_DIR := ./ml_compiler
COMPILER     := $(COMPILER_DIR)/target/debug/ml_compiler
BUILD_DIR    := ./build

BOOT_S        := ./boot.s
LINKER_SCRIPT := ./link.ld

CFLAGS  := -march=rv32i -mabi=ilp32 -O0 -nostdlib -ffreestanding
LDFLAGS := -T $(LINKER_SCRIPT) -Wl,--no-warn-rwx-segments

SRCS := $(wildcard src/*.ml)
NAMES := $(patsubst src/%.ml,%,$(SRCS))
S_FILES   := $(patsubst %,$(BUILD_DIR)/%.s,$(NAMES))
ELF_FILES := $(patsubst %,$(BUILD_DIR)/%.elf,$(NAMES))
BIN_FILES := $(patsubst %,$(BUILD_DIR)/%.bin,$(NAMES))

RUN_TARGETS  := $(addprefix run-,$(NAMES))
TEST_TARGETS := $(addprefix test-,$(NAMES))

.PHONY: all cpu compiler clean $(RUN_TARGETS) $(TEST_TARGETS)
.SECONDARY:

all: cpu compiler

cpu: $(CPU)

$(CPU):
	$(MAKE) -C $(CPU_DIR)

compiler: $(COMPILER)

$(COMPILER):
	cd $(COMPILER_DIR) && cargo build

$(S_FILES): $(BUILD_DIR)/%.s: src/%.ml $(COMPILER)
	@mkdir -p $(BUILD_DIR)
	$(COMPILER) -i $< -o $@

$(ELF_FILES): $(BUILD_DIR)/%.elf: $(BUILD_DIR)/%.s $(BOOT_S) $(LINKER_SCRIPT)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(BOOT_S) $< $(LDFLAGS) -o $@

$(BIN_FILES): $(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf
	$(OBJCOPY) -O binary $< $@
	$(OBJDUMP) -D $< > $(BUILD_DIR)/$*.dis

$(RUN_TARGETS): run-%: $(CPU) $(BUILD_DIR)/%.bin
	$(CPU) +EXEC=$(BUILD_DIR)/$*.bin \
	       +TRACE_FILE=$(BUILD_DIR)/$*.trace \
	       +LOG_FILE=$(BUILD_DIR)/$*.log \
	       +VCD_FILE=$(BUILD_DIR)/$*.vcd
	cat $(BUILD_DIR)/$*.log

$(TEST_TARGETS): test-%: $(BUILD_DIR)/%.elf $(CPU)
	$(TEST_SH) $(BUILD_DIR)/$*.elf

clean:
	rm -rf $(BUILD_DIR)
	cargo clean --manifest-path $(COMPILER_DIR)/Cargo.toml
	$(MAKE) -C $(CPU_DIR) clean