SRCS := $(wildcard ./*.sv) $(wildcard ./modules/*.sv)

BUILD_DIR := ../../build

FORMATTER := verible-verilog-format --inplace
LINTER := verilator --lint-only
VVP := $(BUILD_DIR)/rv32i.vvp
FLAGS := -g2012

.PHONY: all lint format

all: format lint $(VVP)

format:
	$(FORMATTER) $(SRCS)

lint:
	$(LINTER) $(SRCS)

$(VVP): $(SRCS)
	mkdir -p $(BUILD_DIR)
	iverilog $(FLAGS) -o $(VVP) $(SRCS)