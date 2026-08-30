typedef enum logic [4:0] {A = 1} alu_type_e;

module alu (
    input  logic [31:0] a,
    input  logic [31:0] b,
    input  logic [ 4:0] op_type,
    output logic [31:0] out
);

  always_comb begin
    case (op_type)
      A: out = a;
    endcase
  end

endmodule
