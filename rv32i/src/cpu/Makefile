SRCS := $(wildcard ./*.sv) $(wildcard ./modules/*.sv)

BUILD_DIR := ../../build

VVP := $(BUILD_DIR)/rv32i.vvp
FLAGS := -g2012
FORMATTER := verible-verilog-format --inplace
LINTER := verilator --lint-only

all:
	$(LINTER) $(SRCS)
	$(FORMATTER) $(SRCS)
	iverilog $(FLAGS) -o $(VVP) $(SRCS)
