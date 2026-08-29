SRCS := $(wildcard ./*.sv)

BUILD_DIR := ../../build

VVP := $(BUILD_DIR)/rv32i.vvp
FLAGS := -g2012

all:
	iverilog $(FLAGS) -o $(VVP) $(SRCS)
