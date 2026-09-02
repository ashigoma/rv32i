
// RAM
// async read, sync write
// 0x9000_0000 ~ 0x9000_FFFF

module ram (
    input logic clk,
    input logic we,
    input logic [31:0] adr,
    input logic [31:0] wdata,
    output logic [31:0] data
);
  localparam ADR_START = 32'h90000000;
  localparam MEM_SIZE = 32'h00010000;

  logic [7:0] mem[256 * 256];
  logic [31:0] adr_offset;

  always_ff @(posedge clk) begin
    begin
      if (we) begin
        if (ADR_START <= adr && adr < ADR_START + MEM_SIZE) begin
          mem[adr_offset]   <= wdata[31:24];
          mem[adr_offset+1] <= wdata[23:16];
          mem[adr_offset+2] <= wdata[15:8];
          mem[adr_offset+3] <= wdata[7:0];
        end
      end
    end
  end

  always_comb begin
    adr_offset = adr - ADR_START;
    data = {mem[adr_offset], mem[adr_offset+1], mem[adr_offset+2], mem[adr_offset+3]};
  end

endmodule
