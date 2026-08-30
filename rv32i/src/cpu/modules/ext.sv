typedef enum logic [2:0] {
  NONE = 1,
  ZERO_HALF = 2,
  ZERO_BYTE = 3,
  SIGN_HALF = 4,
  SIGN_BYTE = 5
} ext_type_e;

module ext (
    input logic [31:0] data,
    input ext_type_e ext_type,
    output logic [31:0] out
);

  logic [15:0] half_data = data[15:0];
  logic [ 7:0] byte_data = data[7:0];

  always_comb begin
    case (ext_type)
      NONE: out = data;
      ZERO_HALF: out = {16'b0, half_data};
      ZERO_BYTE: out = {24'b0, byte_data};
      SIGN_HALF: out = 32'($signed(half_data));
      SIGN_BYTE: out = 32'($signed(byte_data));
      default: out = 32'hCCCCCCCC;
    endcase
  end

endmodule
