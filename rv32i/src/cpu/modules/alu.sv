`include "types.svh"

module alu (
    input  logic [31:0] a,
    input  logic [31:0] b,
    input  logic [ 4:0] op_type,
    output logic [31:0] out
);

  always_comb begin
    case (op_type)
      ALU_A: out = a;
      ALU_ADD: out = a + b;
    endcase
  end

endmodule
