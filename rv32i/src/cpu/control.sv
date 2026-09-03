`include "types.svh"

module control (
    input logic [31:0] inst,
    input op_e op,
    output sel_e sel_1,
    output sel_e sel_2,
    output sel_e sel_3,
    output alu_type_e alu,
    output ext_type_e ext,
    output comb_type_e comb,
    output logic skip_ram,
    output logic link_reg,
    output logic ram_we,
    output logic reg_we
);

  // see also: branch control (rv32i.sv)
  always_comb begin
    case (op)
      OP_LUI: begin
        // rd = imm_u
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_IMM_U, SEL_NONE};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b0, 1'b1};
      end
      OP_AUIPC: begin
        // rd = imm_u + pc
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_IMM_U, SEL_PC};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b0, 1'b1};
      end
      OP_JAL: begin
        // rd = pc + 4, pc = pc + imm_j
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_PC, SEL_IMM_J};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b1, EXT_NONE, 1'b0, 1'b1};
      end
      OP_JALR: begin
        // rd = pc + 4, pc = rs1 + imm_i
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_I};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b1, EXT_NONE, 1'b0, 1'b1};
      end
      OP_LB: begin
        // rd = [rs1 + imm_i] (load byte signed)
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_I};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b0, 1'b0, EXT_SIGN_BYTE, 1'b0, 1'b1};
      end
      OP_LW: begin
        // rd = [rs1 + imm_i]
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_I};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b0, 1'b0, EXT_NONE, 1'b0, 1'b1};
      end
      OP_LBU: begin
        // rd = [rs1 + imm_i] (load byte unsigned)
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_I};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b0, 1'b0, EXT_ZERO_BYTE, 1'b0, 1'b1};
      end
      OP_SB: begin
        // [rs1 + imm_s] = rs2 (store byte)
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_S};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b1, 1'b0};
        {sel_3, comb} = {SEL_R2, COMB_BYTE};
      end
      OP_SW: begin
        // [rs1 + imm_s] = rs2
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_S};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b1, 1'b0};
        {sel_3, comb} = {SEL_R2, COMB_L};
      end
      OP_ADDI: begin
        // rd = r1 + imm_i
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_IMM_I};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b0, 1'b1};
      end
      OP_ADD: begin
        // rd = r1 + r2
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R1, SEL_R2};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b0, 1'b1};
      end
      default: begin
        {alu, sel_1, sel_2} = {ALU_A, SEL_R1, SEL_R1};
        {sel_3, comb} = {SEL_R1, COMB_L};
        {skip_ram, link_reg, ext, ram_we, reg_we} = {1'b1, 1'b0, EXT_NONE, 1'b0, 1'b0};
      end
    endcase
  end

endmodule
