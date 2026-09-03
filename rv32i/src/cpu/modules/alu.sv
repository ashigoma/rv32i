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
      ALU_OR: out = a | b;
      ALU_XOR: out = a ^ b;
      ALU_SLT: out = ($signed(a[11:0]) < $signed(b[11:0])) ? 32'b1 : 32'b0;
      ALU_SLTU: out = ($unsigned(a[11:0]) < $unsigned(b[11:0])) ? 32'b1 : 32'b0;
      ALU_SLL: out = a << b[4:0];
      ALU_SRL: out = a >> b[4:0];
      ALU_SRA: out = $signed(a) >>> b[4:0];
    endcase
  end

endmodule
