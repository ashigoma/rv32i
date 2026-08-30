SRCS := $(wildcard ./*.sv)

BUILD_DIR := ../../build

VVP := $(BUILD_DIR)/rv32i.vvp
FLAGS := -g2012
FORMATTER := verible-verilog-format --inplace

all:
	$(FORMATTER) $(SRCS)
	iverilog $(FLAGS) -o $(VVP) $(SRCS)
