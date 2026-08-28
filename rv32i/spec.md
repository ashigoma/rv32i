# 入出力
入力は無し
標準出力はMMIOから
cpu例外は標準出力に行く
トレースログは自動でlogfileへ

# [RV32I Base Integer Instruction Set](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html)
## レジスタ
- 32bit
- x0 ~ x31 + pc
- x0は全bit 0

## 命令フォーマット
- 32bit 固定長
- rs1, rs2 = source, rd = destination
- immはすべて符号拡張
- R, I, S, U + B, J (後半2つはshift)

## 標準命令
ADDI SLTI SLTIU ANDI ORI XORI
SLLI SRLI SRAI
LUI AUIPC
ADD SLT SLTU AND OR XOR SLL SRL SUB SRA
JAL
JALR
BEQ BNE BLT BLTU BGE BGEU
LOAD STORE (byte長variantあり)