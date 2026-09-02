`include "types.svh"

// selector

module selector (
    input logic [31:0] pc,
    input logic [31:0] imm_i,
    input logic [31:0] imm_s,
    input logic [31:0] imm_b,
    input logic [31:0] imm_u,
    input logic [31:0] imm_j,
    input logic [31:0] r1,
    input logic [31:0] r2,
    input sel_e sel,
    output logic [31:0] out
);
  always_comb begin
    case (sel)
      SEL_NONE: out = '0;
      SEL_PC: out = pc;
      SEL_IMM_I: out = imm_i;
      SEL_IMM_S: out = imm_s;
      SEL_IMM_B: out = imm_b;
      SEL_IMM_U: out = imm_u;
      SEL_IMM_J: out = imm_j;
      SEL_R1: out = r1;
      SEL_R2: out = r2;
      default: out = 32'hCCCCCCCC;
    endcase
  end
endmodule
