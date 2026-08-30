typedef enum logic [4:0] {
  L = 1,
  HALF = 2,
  BYTE = 3,
  H = 4
} comb_type_e;

module comb (
    input  logic [31:0] l,
    input  logic [31:0] h,
    input  logic [ 4:0] comb_type,
    output logic [31:0] out
);
  logic [15:0] h_half = h[31:16];
  logic [23:0] h_byte = h[31:8];
  logic [15:0] l_half = l[15:0];
  logic [ 7:0] l_byte = l[7:0];

  always_comb begin
    case (comb_type)
      L: out = l;
      HALF: out = {h_half, l_half};
      BYTE: out = {h_byte, l_byte};
      H: out = h;
      default: out = 32'hCCCCCCCC;
    endcase
  end

endmodule
