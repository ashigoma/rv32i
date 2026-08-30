// register file
// x0 ~ x31, 32 bits each
// x0 is always set to 0
// async read, sync write

module reg_file (
    // async read
    input  logic [ 4:0] rd1,
    input  logic [ 4:0] rd2,
    output logic [31:0] r1,
    output logic [31:0] r2,

    // sync write
    input logic [4:0] rd3,
    input logic [31:0] wdata,
    input logic we,

    input logic clk,
    input logic rst
);
  logic [31:0] regs[31:0];

  always_ff @(posedge clk, posedge rst) begin
    if (rst) begin
      for (int i = 0; i < 32; i++) begin
        regs[i] <= '0;
      end
    end else begin
      if (we && (rd3 != 5'd0)) begin
        regs[rd3] <= wdata;
      end
    end
  end

  always_comb begin
    r1 = regs[rd1];
    r2 = regs[rd2];
  end
endmodule
