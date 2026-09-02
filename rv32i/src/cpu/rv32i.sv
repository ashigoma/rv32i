`include "types.svh"

module rv32i (
    input logic clk,
    input logic rst,
    input int   trace_fd,
    input int   log_fd
);
  string exec_file;
  logic [31:0] mem[0:255];
  int bin_file;

  logic [31:0] pc_reg;
  logic [31:0] pc_next, pc_in, pc, inst, wb;
  logic [31:0] imm_i, imm_s, imm_b, imm_u, imm_j;

  wire [11:0] inst_31_20 = inst[31:20];
  wire [6:0] inst_31_25 = inst[31:25];
  wire [4:0] inst_11_7 = inst[11:7];
  wire [5:0] inst_30_25 = inst[30:25];
  wire [3:0] inst_11_8 = inst[11:8];
  wire [19:0] inst_31_12 = inst[31:12];
  wire [7:0] inst_19_12 = inst[19:12];
  wire [9:0] inst_30_21 = inst[30:21];
  wire inst_31 = inst[31];
  wire inst_20 = inst[20];
  wire inst_7 = inst[7];

  op_e op;

  sel_e ctl_sel_1, ctl_sel_2, ctl_sel_3;
  alu_type_e  ctl_alu;
  ext_type_e  ctl_ext;
  comb_type_e ctl_comb;
  logic ctl_branch, ctl_reg_we, ctl_ram_we, ctl_skip_ram;

  // register file access
  wire [4:0] rs1 = inst[19:15];
  wire [4:0] rs2 = inst[24:20];
  wire [4:0] rs3 = inst[11:7];

  logic [31:0] r1, r2;
  logic [31:0] alu_a, alu_b, d3;
  logic [31:0] alu_out;
  logic [31:0] ram_wdata, ram_out;
  logic [31:0] ext_in;

  op_decode op_decode_ (
      .inst(inst),
      .op  (op)
  );

  control control_ (
      .inst(inst),
      .op(op),
      .sel_1(ctl_sel_1),
      .sel_2(ctl_sel_2),
      .sel_3(ctl_sel_3),
      .alu(ctl_alu),
      .ext(ctl_ext),
      .comb(ctl_comb),
      .skip_ram(ctl_skip_ram),
      .ram_we(ctl_ram_we),
      .reg_we(ctl_reg_we)
  );

  reg_file reg_file_ (
      .rs1(rs1),
      .rs2(rs2),
      .r1 (r1),
      .r2 (r2),

      .rs3(rs3),
      .wdata(wb),
      .we(ctl_reg_we),

      .clk(clk),
      .rst(rst)
  );

  selector selector_1_ (
      .pc(pc),
      .imm_i(imm_i),
      .imm_s(imm_s),
      .imm_b(imm_b),
      .imm_u(imm_u),
      .imm_j(imm_j),
      .r1(r1),
      .r2(r2),
      .sel(ctl_sel_1),
      .out(alu_a)
  );
  selector selector_2_ (
      .pc(pc),
      .imm_i(imm_i),
      .imm_s(imm_s),
      .imm_b(imm_b),
      .imm_u(imm_u),
      .imm_j(imm_j),
      .r1(r1),
      .r2(r2),
      .sel(ctl_sel_2),
      .out(alu_b)
  );
  selector selector_3_ (
      .pc(pc),
      .imm_i(imm_i),
      .imm_s(imm_s),
      .imm_b(imm_b),
      .imm_u(imm_u),
      .imm_j(imm_j),
      .r1(r1),
      .r2(r2),
      .sel(ctl_sel_3),
      .out(d3)
  );
  alu alu_ (
      .a(alu_a),
      .b(alu_b),
      .op_type(ctl_alu),
      .out(alu_out)
  );

  ram ram_ (
      .clk(clk),
      .we(ctl_ram_we),
      .adr(alu_out),
      .wdata(ram_wdata),
      .data(ram_out)
  );

  comb comb_ (
      .l(d3),
      .h(wb),
      .comb_type(ctl_comb),
      .out(ram_wdata)
  );

  ext ext_ (
      .data(ext_in),
      .ext_type(ctl_ext),
      .out(wb)
  );

  initial begin
    if (!$value$plusargs("EXEC=%s", exec_file)) begin
      exec_file = "program.bin";
      #1;

      bin_file = $fopen(exec_file, "rb");
      $fread(mem, bin_file);
      $fclose(bin_file);
    end else begin
      $fdisplay(log_fd, "Error: Cannot open %s", exec_file);
    end

    for (int i = 0; i < 256; i++) begin
      if (mem[i] !== 32'bx) begin
        $fdisplay(trace_fd, "%08h: %08h", i * 4, mem[i]);
      end
    end
  end

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      pc_reg <= 32'h80000000;
    end else begin
      pc_reg <= pc_in;
    end
  end

  always_comb begin
    // instruction fetch
    inst = mem[pc-32'h80000000];
    pc_next = pc + 32'h00000004;
    if (ctl_branch) begin
      pc_in = wb;
    end else begin
      pc_in = pc_next;
    end

    // imm
    imm_i = 32'($signed(inst_31_20));
    imm_s = 32'($signed({inst_31_25, inst_11_7}));
    imm_b = 32'($signed({inst_31, inst_7, inst_30_25, inst_11_8}));
    imm_u = 32'($signed(inst_31_12));
    imm_j = 32'($signed({inst_31, inst_19_12, inst_20, inst_30_21}));

    // skip ram
    if (ctl_skip_ram) begin
      ext_in = alu_out;
    end else begin
      ext_in = ram_out;
    end

  end

endmodule
