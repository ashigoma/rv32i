CC := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
OBJDUMP := riscv64-unknown-elf-objdump

TEST_DIR := .
LIB_DIR := ../../lib
BUILD_DIR := ../../build
TEST_SCRIPT := ./scripts/test.sh

CRT0 := $(LIB_DIR)/crt0.S
LIB_SRCS := $(filter-out $(CRT0),$(wildcard $(LIB_DIR)/*.c) $(wildcard $(LIB_DIR)/*.S))
TEST_SRCS := $(wildcard $(TEST_DIR)/*.c) $(wildcard $(TEST_DIR)/*.S)

ALL_BINS := $(patsubst $(TEST_DIR)/%, $(BUILD_DIR)/%.bin, $(basename $(TEST_SRCS)))
ALL_ELFS := $(patsubst $(TEST_DIR)/%, $(BUILD_DIR)/%.elf, $(basename $(TEST_SRCS)))

CFLAGS := -march=rv32i -mabi=ilp32 -O0 -nostdlib -ffreestanding -I$(LIB_DIR)
LDFLAGS := -T ../../link.ld -Wl,--no-warn-rwx-segments

.PHONY: all check clean
.SECONDARY:

all: $(ALL_ELFS) $(ALL_BINS)

check: all
	@for bin in $(ALL_ELFS); do \
		$(TEST_SCRIPT) $$bin || exit 1; \
	done

$(BUILD_DIR)/%.elf: $(TEST_DIR)/%.c $(CRT0) $(LIB_SRCS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(CRT0) $< $(filter-out $<, $(LIB_SRCS)) $(LDFLAGS) -o $@

$(BUILD_DIR)/%.elf: $(TEST_DIR)/%.S $(CRT0) $(LIB_SRCS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) $(CRT0) $< $(filter-out $<, $(LIB_SRCS)) $(LDFLAGS) -o $@

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf
	$(OBJCOPY) -O binary $< $@
	$(OBJDUMP) -d $< > $(BUILD_DIR)/$*.dis