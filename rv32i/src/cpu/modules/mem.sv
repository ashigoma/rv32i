// ROM (async read) 0x8000_0000 ~ 0x8000_FFFF
// RAM (async read, sync write) 0x9000_0000 ~ 0x9000_FFFF

module mem (
    input logic clk,
    input logic we2,
    input logic [31:0] adr1,
    input logic [31:0] adr2,
    input logic [31:0] wdata,
    output logic [31:0] data1,
    output logic [31:0] data2
);

  // instruction memory (read only)
  localparam ADR_START_1 = 32'h80000000;
  localparam MEM_SIZE_1 = 32'h00010004;

  // data memory (read / write)
  localparam ADR_START_2 = 32'h90000000;
  localparam MEM_SIZE_2 = 32'h00010004;

  logic [7:0] mem1[MEM_SIZE_1] = '{default: 8'h00};
  logic [7:0] mem2[MEM_SIZE_2] = '{default: 8'h00};
  logic [31:0] adr_offset_1;
  logic [31:0] adr_offset_2_inst;
  logic [31:0] adr_offset_2_data;

  initial begin
    string exec_file;
    int read_bytes;
    int fd;

    $display("init mem");
    if (!$value$plusargs("EXEC=%s", exec_file)) exec_file = "program.bin";

    fd = $fopen(exec_file, "rb");
    if (fd == 0) begin
      $display("error: cannot open file: %s", exec_file);
    end else begin
      read_bytes = $fread(mem1, fd);
      $fclose(fd);
      $display("loaded %0d bytes from %s", read_bytes, exec_file);
    end
  end

  always_ff @(posedge clk) begin
    begin
      if (we2) begin
        if (ADR_START_2 <= adr2 && adr2 < ADR_START_2 + MEM_SIZE_2) begin
          mem2[adr_offset_2_data+3] <= wdata[31:24];
          mem2[adr_offset_2_data+2] <= wdata[23:16];
          mem2[adr_offset_2_data+1] <= wdata[15:8];
          mem2[adr_offset_2_data]   <= wdata[7:0];
        end
      end
    end
  end

  always_comb begin
    // instruction read
    adr_offset_1 = adr1 - ADR_START_1;
    data1 = {mem1[adr_offset_1+3], mem1[adr_offset_1+2], mem1[adr_offset_1+1], mem1[adr_offset_1]};

    adr_offset_2_inst = adr2 - ADR_START_1;
    adr_offset_2_data = adr2 - ADR_START_2;

    // data read
    if (ADR_START_1 <= adr2 && adr2 < ADR_START_1 + MEM_SIZE_1) begin
      data2 = {
        mem1[adr_offset_2_inst+3],
        mem1[adr_offset_2_inst+2],
        mem1[adr_offset_2_inst+1],
        mem1[adr_offset_2_inst]
      };
    end else if (ADR_START_2 <= adr2 && adr2 < ADR_START_2 + MEM_SIZE_2) begin
      data2 = {
        mem2[adr_offset_2_data+3],
        mem2[adr_offset_2_data+2],
        mem2[adr_offset_2_data+1],
        mem2[adr_offset_2_data]
      };
    end else begin
      data2 = '0;
    end
  end

endmodule
