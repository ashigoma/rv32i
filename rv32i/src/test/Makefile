CC := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
OBJDUMP := riscv64-unknown-elf-objdump

TEST_DIR := .
LIB_DIR := ../../lib
BUILD_DIR := ../../build
TEST_SCRIPT := ./scripts/test.sh

BOOT_S := $(LIB_DIR)/boot.S
LIB_SRCS := $(filter-out $(BOOT_S),$(wildcard $(LIB_DIR)/*.c) $(wildcard $(LIB_DIR)/*.S))
TEST_SRCS := $(wildcard $(TEST_DIR)/*.c) $(wildcard $(TEST_DIR)/*.S)
LINKER_SCRIPT := ../../link.ld

ALL_BINS := $(patsubst $(TEST_DIR)/%, $(BUILD_DIR)/%.bin, $(basename $(TEST_SRCS)))
ALL_ELFS := $(patsubst $(TEST_DIR)/%, $(BUILD_DIR)/%.elf, $(basename $(TEST_SRCS)))

CFLAGS := -march=rv32i -mabi=ilp32 -O0 -nostdlib -ffreestanding -I$(LIB_DIR)
LDFLAGS := -T $(LINKER_SCRIPT) -Wl,--no-warn-rwx-segments

.PHONY: all check clean
.SECONDARY:

all: $(ALL_ELFS) $(ALL_BINS)

check: all
	for bin in $(BUILD_DIR)/*.elf; do \
		$(TEST_SCRIPT) $$bin || exit 1; \
	done

$(BUILD_DIR)/%.elf: $(TEST_DIR)/%.c $(BOOT_S) $(LIB_SRCS) $(LINKER_SCRIPT)
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(BOOT_S) $< $(filter-out $<, $(LIB_SRCS)) $(LDFLAGS) -o $@

$(BUILD_DIR)/%.elf: $(TEST_DIR)/%.S $(BOOT_S) $(LIB_SRCS) $(LINKER_SCRIPT)
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(BOOT_S) $< $(filter-out $<, $(LIB_SRCS)) $(LDFLAGS) -o $@

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf
	$(OBJCOPY) -O binary $< $@
	$(OBJDUMP) -D $< > $(BUILD_DIR)/$*.dis