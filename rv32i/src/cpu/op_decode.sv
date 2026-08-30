typedef enum {
    OP_LUI,
    OP_AUIPC,
    OP_JAL,
    OP_JALR,
    OP_BEQ,
    OP_BNE,
    OP_BLT,
    OP_BGE,
    OP_BLTU,
    OP_BGEU,
    OP_LB,
    OP_LH,
    OP_LW,
    OP_LBU,
    OP_LHU,
    OP_SB,
    OP_SH,
    OP_SW,
    OP_ADDI,
    OP_SLTI,
    OP_SLTIU,
    OP_XORI,
    OP_ORI,
    OP_ANDI,
    OP_SLLI,
    OP_SRLI,
    OP_SRAI,
    OP_ADD,
    OP_SUB,
    OP_SLL,
    OP_SLT,
    OP_SLTU,
    OP_XOR,
    OP_SRL,
    OP_SRA,
    OP_OR,
    OP_AND,
    OP_FENCE,
    OP_FENCE_TSO,
    OP_PAUSE,
    OP_ECALL,
    OP_EBREAK,
} op_e;

module op_decode (
    input logic[31:0] inst;
    output op_e op;
);

    logic [6:0] opcode = inst[6:0];
    locic [2:0] funct3 = inst[14:12];
    locic [6:0] funct7 = inst[31:25];

    always_comb begin
        case (opcode) begin
            7'b0110111: op = OP_LUI;
            
            default: op = OP_EBREAK;
        endcase
    end
endmodule