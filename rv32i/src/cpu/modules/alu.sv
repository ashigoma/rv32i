`include "types.svh"


module alu (
    input logic [31:0] a,
    input logic [31:0] b,
    input alu_type_e op_type,
    output logic [31:0] out
);

  always_comb begin
    case (op_type)
      ALU_NONE: out = '0;
      ALU_A: out = a;
      ALU_ADD: out = a + b;
      ALU_SUB: out = a - b;
      ALU_AND: out = a & b;
      ALU_XOR: out = a ^ b;
      ALU_SLT: out = (a < b) ? 32'b1 : 32'b0;
    endcase
  end

endmodule
