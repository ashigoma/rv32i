`include "types.svh"


module ext (
    input logic [31:0] data,
    input ext_type_e ext_type,
    output logic [31:0] out
);

  wire [15:0] half_data = data[15:0];
  wire [ 7:0] byte_data = data[7:0];

  always_comb begin
    case (ext_type)
      EXT_NONE: out = data;
      EXT_ZERO_HALF: out = {16'b0, half_data};
      EXT_ZERO_BYTE: out = {24'b0, byte_data};
      EXT_SIGN_HALF: out = 32'($signed(half_data));
      EXT_SIGN_BYTE: out = 32'($signed(byte_data));
    endcase
  end

endmodule
