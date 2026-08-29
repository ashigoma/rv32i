# 入出力
入力は無し
標準出力はMMIOから
cpu例外は標準出力に行く
トレースログは自動でlogfileへ

# メモリマップ
- 0x2000_0000 ~ 0x2000_0FFF : RAM
    - 標準出力系, fromhost / tohost
- 0x8000_0000 ~ 0x8FFF_FFFF : ROM
    - .text.init エントリポイント
    - .text *.text プログラム
    - .rodata 定数
    - .data 初期値あり変数の初期値
- 0x9000_0000 ~ 0x9FFF_FFFF : RAM
    - .data 初期値あり変数の実体
    - .bss 初期値なし変数
    - stack (上から)

# エントリポイント
boot.s
- .dataの初期値をRAMへコピー
- .bssをクリア
- spをセット

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