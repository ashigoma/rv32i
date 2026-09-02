SRCS := $(wildcard ./*.sv) $(wildcard ./modules/*.sv)

BUILD_DIR := ../../build

FORMATTER := verible-verilog-format --inplace
LINTER := verilator --lint-only
VVP := $(BUILD_DIR)/rv32i.vvp
FLAGS := -g2012

all:
	$(LINTER) $(SRCS)
	$(FORMATTER) $(SRCS)
	iverilog $(FLAGS) -o $(VVP) $(SRCS)
