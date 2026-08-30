typedef enum logic [4:0] {
  PC = 1,
  IMM_I = 2,
  IMM_S = 3,
  IMM_B = 4,
  IMM_U = 5,
  IMM_J = 6,
  R1 = 7,
  R2 = 8
} sel_e;

module selector (
    input  logic [31:0] pc,
    input  logic [31:0] imm_i,
    input  logic [31:0] imm_s,
    input  logic [31:0] imm_b,
    input  logic [31:0] imm_u,
    input  logic [31:0] imm_j,
    input  logic [31:0] r1,
    input  logic [31:0] r2,
    input  logic [ 4:0] sel,
    output logic [31:0] out
);
  always_comb begin
    case (sel)
      PC: out = pc;
      IMM_I: out = imm_i;
      IMM_S: out = imm_s;
      IMM_B: out = imm_b;
      IMM_U: out = imm_u;
      IMM_J: out = imm_j;
      R1: out = r1;
      R2: out = r2;
      default: out = 31'd0;
    endcase
  end
endmodule
