SRCS := $(wildcard ./*.sv) $(wildcard ./modules/*.sv)

BUILD_DIR := ../../build

FORMATTER := verible-verilog-format --inplace
LINTER := verilator --lint-only
FLAGS := -g2012

.PHONY: all lint format

all: format lint main

format:
	$(FORMATTER) $(SRCS)

lint:
	$(LINTER) $(SRCS)

main: $(SRCS)
	mkdir -p $(BUILD_DIR)
	verilator --binary -j 0 --Mdir $(BUILD_DIR) -o main --top-module main $(SRCS)