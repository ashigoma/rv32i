`ifndef TYPES_SVH
`define TYPES_SVH

typedef enum {
  SEL_NONE,
  SEL_PC,
  SEL_IMM_I,
  SEL_IMM_S,
  SEL_IMM_B,
  SEL_IMM_U,
  SEL_IMM_J,
  SEL_R1,
  SEL_R2
} sel_e;

typedef enum {
  EXT_NONE,
  EXT_ZERO_HALF,
  EXT_ZERO_BYTE,
  EXT_SIGN_HALF,
  EXT_SIGN_BYTE
} ext_type_e;

typedef enum {
  COMB_NONE,
  COMB_L,
  COMB_HALF,
  COMB_BYTE,
  COMB_H
} comb_type_e;

typedef enum {
  ALU_NONE,
  ALU_A,
  ALU_ADD,
  ALU_SUB,
  ALU_AND,
  ALU_OR,
  ALU_XOR,
  ALU_SLT,
  ALU_SLTU,
  ALU_SLL,
  ALU_SRL,
  ALU_SRA
} alu_type_e;

typedef enum {
  OP_NONE,
  OP_LUI,
  OP_AUIPC,
  OP_JAL,
  OP_JALR,
  OP_BEQ,
  OP_BNE,
  OP_BLT,
  OP_BGE,
  OP_BLTU,
  OP_BGEU,
  OP_LB,
  OP_LH,
  OP_LW,
  OP_LBU,
  OP_LHU,
  OP_SB,
  OP_SH,
  OP_SW,
  OP_ADDI,
  OP_SLTI,
  OP_SLTIU,
  OP_XORI,
  OP_ORI,
  OP_ANDI,
  OP_SLLI,
  OP_SRLI,
  OP_SRAI,
  OP_ADD,
  OP_SUB,
  OP_SLL,
  OP_SLT,
  OP_SLTU,
  OP_XOR,
  OP_SRL,
  OP_SRA,
  OP_OR,
  OP_AND,
  OP_FENCE,
  OP_FENCE_TSO,
  OP_PAUSE,
  OP_ECALL,
  OP_EBREAK
} op_e;

`endif