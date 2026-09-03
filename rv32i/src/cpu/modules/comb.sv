`include "types.svh"


module comb (
    input logic [31:0] l,
    input logic [31:0] h,
    input comb_type_e comb_type,
    output logic [31:0] out
);
  wire [15:0] h_half = h[31:16];
  wire [23:0] h_byte = h[31:8];
  wire [15:0] l_half = l[15:0];
  wire [ 7:0] l_byte = l[7:0];

  always_comb begin
    case (comb_type)
      COMB_NONE: out = '0;
      COMB_L: out = l;
      COMB_HALF: out = {h_half, l_half};
      COMB_BYTE: out = {h_byte, l_byte};
      COMB_H: out = h;
    endcase
  end

endmodule
