`include "types.svh"

module rv32i (
    input logic clk,
    input logic rst,
    input int   trace_fd,
    input int   log_fd
);

  logic [31:0] pc;
  logic [31:0] pc_next, pc_in, inst, wb;
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
  logic ctl_link_reg;

  wire [4:0] rs1 = inst[19:15];
  wire [4:0] rs2 = inst[24:20];
  wire [4:0] rs3 = inst[11:7];
  logic [31:0] reg_wdata;

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
      .link_reg(ctl_link_reg),
      .ram_we(ctl_ram_we),
      .reg_we(ctl_reg_we)
  );

  reg_file reg_file_ (
      .rs1(rs1),
      .rs2(rs2),
      .r1 (r1),
      .r2 (r2),

      .rs3(rs3),
      .wdata(reg_wdata),
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

  mem mem_ (
      .clk  (clk),
      .we2  (ctl_ram_we),
      .adr1 (pc),
      .adr2 (alu_out),
      .data1(inst),
      .data2(ram_out),
      .wdata(ram_wdata)
  );

  comb comb_ (
      .l(d3),
      .h(ram_out),
      .comb_type(ctl_comb),
      .out(ram_wdata)
  );

  ext ext_ (
      .data(ext_in),
      .ext_type(ctl_ext),
      .out(wb)
  );

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      pc <= 32'h80000000;
    end else begin
      pc <= pc_in;
    end
  end

  always @(posedge clk) begin
    if (!rst) begin
      if (op == OP_EBREAK) begin  // halt
        $finish;
      end else if (ctl_reg_we) begin  // trace log
        if (rs3 == '0) begin
        end else if (ctl_skip_ram) begin
          $fdisplay(trace_fd, "(0x%08h) x%0d = 0x%08h", pc, rs3, reg_wdata);
        end else begin
          $fdisplay(trace_fd, "(0x%08h) x%0d = [0x%08h]", pc, rs3, alu_out);
        end
      end else if (ctl_ram_we) begin
        case (ctl_comb)
          COMB_BYTE: $fdisplay(trace_fd, "(0x%08h) [0x%08h] = 0x%02h", pc, alu_out, ram_wdata[7:0]);
          COMB_HALF:
          $fdisplay(trace_fd, "(0x%08h) [0x%08h] = 0x%04h", pc, alu_out, ram_wdata[15:0]);
          default: $fdisplay(trace_fd, "(0x%08h) [0x%08h] = 0x%08h", pc, alu_out, ram_wdata);
        endcase
      end else if (ctl_alu == ALU_NONE) begin
        $fdisplay(trace_fd, "(0x%08h) inst: 0x%08h", pc, inst);
      end

      if (ctl_ram_we) begin
        if (alu_out == 32'h20000000) begin
          $fwrite(log_fd, "%c", ram_wdata[7:0]);
        end else if (alu_out == 32'h20000004) begin
          $fwrite(log_fd, "%08h", ram_wdata);
        end else if (alu_out == 32'h20000008) begin
          $fwrite(log_fd, "%0d", ram_wdata);
        end
        $fflush(log_fd);
      end
    end
  end

  always_comb begin
    // branch control
    case (op)
      OP_JAL:  ctl_branch = 1'b1;
      OP_JALR: ctl_branch = 1'b1;
      OP_BNE:  ctl_branch = (r1 != r2);
      OP_BLT:  ctl_branch = ($signed(r1) < $signed(r2));
      OP_BGE:  ctl_branch = ($signed(r1) >= $signed(r2));
      default: ctl_branch = 1'b0;
    endcase

    pc_next = pc + 32'h00000004;
    if (ctl_branch) begin
      pc_in = wb;
    end else begin
      pc_in = pc_next;
    end

    // imm
    imm_i = 32'($signed(inst_31_20));
    imm_s = 32'($signed({inst_31_25, inst_11_7}));
    imm_b = 32'($signed({inst_31, inst_7, inst_30_25, inst_11_8, 1'b0}));
    imm_u = 32'(inst_31_12 << 12);
    imm_j = 32'($signed({inst_31, inst_19_12, inst_20, inst_30_21, 1'b0}));

    // skip ram
    if (ctl_skip_ram) begin
      ext_in = alu_out;
    end else begin
      ext_in = ram_out;
    end

    // register write back
    if (ctl_link_reg) begin
      reg_wdata = pc_next;
    end else begin
      reg_wdata = wb;
    end
  end

endmodule
