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
    output logic ram_we,
    output logic reg_we
);

  always_comb begin
    case (op)
      OP_ADDI: begin
        // x1 = x2 + imm_i
        {alu, sel_1, sel_2} = {ALU_ADD, SEL_R2, SEL_IMM_I};
        {sel_3, comb} = {SEL_R1, COMB_L};
        {skip_ram, ext, ram_we, reg_we} = {1'b1, EXT_NONE, 1'b0, 1'b1};
      end
      default: begin
        {alu, sel_1, sel_2} = {ALU_A, SEL_R1, SEL_R1};
        {sel_3, comb} = {SEL_R1, COMB_L};
        {skip_ram, ext, ram_we, reg_we} = {1'b1, EXT_NONE, 1'b0, 1'b0};
      end
    endcase
  end

endmodule
