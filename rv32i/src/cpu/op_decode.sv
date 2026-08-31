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
  OP_NONE
} op_e;

module op_decode (
    input logic [31:0] inst,
    output op_e op
);

  logic [6:0] opcode = inst[6:0];
  logic [2:0] funct3 = inst[14:12];
  logic [6:0] funct7 = inst[31:25];

  always_comb begin
    case (opcode)
      7'b0110111: op = OP_LUI;
      7'b0010111: op = OP_AUIPC;
      7'b1101111: op = OP_JAL;
      7'b1100111: op = OP_JALR;
      7'b1100011: begin
        case (funct3)
          3'b000:  op = OP_BEQ;
          3'b001:  op = OP_BNE;
          3'b100:  op = OP_BLT;
          3'b101:  op = OP_BGE;
          3'b110:  op = OP_BLTU;
          3'b111:  op = OP_BGEU;
          default: op = OP_NONE;
        endcase
      end
      7'b0000011: begin
        case (funct3)
          3'b000:  op = OP_LB;
          3'b001:  op = OP_LH;
          3'b010:  op = OP_LW;
          3'b100:  op = OP_LBU;
          3'b101:  op = OP_LHU;
          default: op = OP_NONE;
        endcase
      end
      7'b0100011: begin
        case (funct3)
          3'b000:  op = OP_SB;
          3'b001:  op = OP_SH;
          3'b010:  op = OP_SW;
          default: op = OP_NONE;
        endcase
      end
      7'b0010011: begin
        case (funct3)
          3'b000:  op = OP_ADDI;
          3'b010:  op = OP_SLTI;
          3'b011:  op = OP_SLTIU;
          3'b100:  op = OP_XORI;
          3'b110:  op = OP_ORI;
          3'b111:  op = OP_ANDI;
          3'b001: begin
            case (funct7)
              7'b0000000: op = OP_SLLI;
              default: op = OP_NONE;
            endcase
          end
          3'b101: begin
            case (funct7)
              7'b0000000: op = OP_SRLI;
              7'b0100000: op = OP_SRAI;
              default: op = OP_NONE;
            endcase
          end
          default: op = OP_NONE;
        endcase
      end
      7'b0110011: begin
        case (funct3)
          3'b000: begin
            case (funct7)
              7'b0000000: op = OP_ADD;
              7'b0100000: op = OP_SUB;
              default: op = OP_NONE;
            endcase
          end
          3'b001:  op = OP_SLL;
          3'b010:  op = OP_SLT;
          3'b011:  op = OP_SLTU;
          3'b100:  op = OP_XOR;
          3'b101: begin
            case (funct7)
              7'b0000000: op = OP_SRL;
              7'b0100000: op = OP_SRA;
              default: op = OP_NONE;
            endcase
          end
          3'b110:  op = OP_OR;
          3'b111:  op = OP_AND;
          default: op = OP_NONE;
        endcase
      end
      7'b0110011: op = OP_PAUSE;  // no FENCE
      7'b1110011: op = OP_EBREAK;  // no ECALL
      default: op = OP_NONE;
    endcase
  end
endmodule
