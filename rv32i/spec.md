
# [RV32I Base Integer Instruction Set](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html)
[アセンブル](http://five-embeddev.com/riscv-user-isa-manual/Priv-v1.12/instr-table.html)
| 命令 | 例 | 操作 | 備考 | 略称 |
| --- | --- | --- | --- | --- |
| LUI | `lui x1, 0x12345` | `x1` ← `0x12345000` | 上位20bitに格納 | **L**oad **U**pper **I**mmediate |
| AUIPC | `auipc x1, 0x12345` | `x1` ← `PC + (0x12345 << 12)` | PC相対で上位20bit加算 | **A**dd **U**pper **I**mmediate to **PC** |
| JAL | `jal x1, label` | `x1` ← `PC + 4`, `PC` ← `label` | 戻り先を保存して相対ジャンプ | **J**ump **A**nd **L**ink |
| JALR | `jalr x1, 4(x2)` | `x1` ← `PC + 4`, `PC` ← `x2 + 4` | 戻り先を保存して絶対ジャンプ | **J**ump **A**nd **L**ink **R**egister |
| BEQ | `beq x1, x2, label` | if (`x1 == x2`) `PC` ← `label` |  | **B**ranch if **EQ**ual |
| BNE | `bne x1, x2, label` | if (`x1 != x2`) `PC` ← `label` |  | **B**ranch if **N**ot **E**qual |
| BLT | `blt x1, x2, label` | if (`x1 < x2` `[signed]`) `PC` ← `label` |  | **B**ranch if **L**ess **T**han |
| BGE | `bge x1, x2, label` | if (`x1 >= x2` `[signed]`) `PC` ← `label` |  | **B**ranch if **G**reater than or **E**qual |
| BLTU | `bltu x1, x2, label` | if (`x1 < x2` `[unsigned]`) `PC` ← `label` |  | **B**ranch if **L**ess **T**han, **U**nsigned |
| BGEU | `bgeu x1, x2, label` | if (`x1 >= x2` `[unsigned]`) `PC` ← `label` |  | **B**ranch if **G**reater than or **E**qual, **U**nsigned |
| LB | `lb x1, 4(x2)` | `x1` ← `SignExt(M[x2 + 4][7:0])` | 1バイト符号拡張ロード | **L**oad **B**yte |
| LH | `lh x1, 4(x2)` | `x1` ← `SignExt(M[x2 + 4][15:0])` | 2バイト符号拡張ロード | **L**oad **H**alfword |
| LW | `lw x1, 4(x2)` | `x1` ← `M[x2 + 4][31:0]` | 4バイトロード | **L**oad **W**ord |
| LBU | `lbu x1, 4(x2)` | `x1` ← `ZeroExt(M[x2 + 4][7:0])` | 1バイトゼロ拡張ロード | **L**oad **B**yte, **U**nsigned |
| LHU | `lhu x1, 4(x2)` | `x1` ← `ZeroExt(M[x2 + 4][15:0])` | 2バイトゼロ拡張ロード | **L**oad **H**alfword, **U**nsigned |
| SB | `sb x1, 4(x2)` | `M[x2 + 4][7:0]` ← `x1[7:0]` | 1バイトストア | **S**tore **B**yte |
| SH | `sh x1, 4(x2)` | `M[x2 + 4][15:0]` ← `x1[15:0]` | 2バイトストア | **S**tore **H**alfword |
| SW | `sw x1, 4(x2)` | `M[x2 + 4][31:0]` ← `x1[31:0]` | 4バイトストア | **S**tore **W**ord |
| ADDI | `addi x1, x2, 1` | `x1` ← `x2 + 1` |  | **ADD** **I**mmediate |
| SLTI | `slti x1, x2, 1` | `x1` ← (`x2 < 1` `[signed]`) ? `1` : `0` |  | **S**et **L**ess **T**han **I**mmediate |
| SLTIU | `sltiu x1, x2, 1` | `x1` ← (`x2 < 1` `[unsigned]`) ? `1` : `0` |  | **S**et **L**ess **T**han **I**mmediate, **U**nsigned |
| XORI | `xori x1, x2, 1` | `x1` ← `x2 ^ 1` |  | **XOR** **I**mmediate |
| ORI | `ori x1, x2, 1` | `x1` ← `x2` \| `1` |  | **OR** **I**mmediate |
| ANDI | `andi x1, x2, 1` | `x1` ← `x2 & 1` |  | **AND** **I**mmediate |
| SLLI | `slli x1, x2, 1` | `x1` ← `x2 << 1` | 論理左シフト | **S**hift **L**eft **L**ogical **I**mmediate |
| SRLI | `srli x1, x2, 1` | `x1` ← `x2 >> 1` | 論理右シフト | **S**hift **R**ight **L**ogical **I**mmediate |
| SRAI | `srai x1, x2, 1` | `x1` ← `x2 >> 1` | 算術右シフト | **S**hift **R**ight **A**rithmetic **I**mmediate |
| ADD | `add x1, x2, x3` | `x1` ← `x2 + x3` |  | **ADD** |
| SUB | `sub x1, x2, x3` | `x1` ← `x2 - x3` |  | **SUB**tract |
| SLL | `sll x1, x2, x3` | `x1` ← `x2 << x3[4:0]` | 論理左シフト | **S**hift **L**eft **L**ogical |
| SLT | `slt x1, x2, x3` | `x1` ← (`x2 < x3` `[signed]`) ? `1` : `0` |  | **S**et **L**ess **T**han |
| SLTU | `sltu x1, x2, x3` | `x1` ← (`x2 < x3` `[unsigned]`) ? `1` : `0` |  | **S**et **L**ess **T**han, **U**nsigned |
| XOR | `xor x1, x2, x3` | `x1` ← `x2 ^ x3` |  | e**X**clusive **OR** |
| SRL | `srl x1, x2, x3` | `x1` ← `x2 >> x3[4:0]` | 論理右シフト | **S**hift **R**ight **L**ogical |
| SRA | `sra x1, x2, x3` | `x1` ← `x2 >> x3[4:0]` | 算術右シフト | **S**hift **R**ight **A**rithmetic |
| OR | `or x1, x2, x3` | `x1` ← `x2` \| `x3` |  | **OR** |
| AND | `and x1, x2, x3` | `x1` ← `x2 & x3` |  | **AND** |
| FENCE | `fence iorw, iorw` | - | メモリ・IO順序制御 | **FENCE** |
| FENCE.TSO | `fence.tso` | - | TSOモデル用メモリフェンス | **FENCE** **T**otal **S**ore **O**rdering |
| PAUSE | `pause` | - | パイプライン一時停止ヒント | **PAUSE** |
| ECALL | `ecall` | - | システムコール発生 | **E**nvironment **CALL** |
| EBREAK | `ebreak` | - | ブレークポイント発生 | **E**nvironment **BREAK** |

# 入出力
入力は無し
標準出力はMMIOから
cpu例外は標準出力に行く
トレースログは自動でlogfileへ

# メモリマップ
- 0x2000_0000 ~ 0x2000_0FFF : RAM
    - 標準出力系, fromhost / tohost
- 0x8000_0000 ~ 0x8000_FFFF : ROM
    - .text.init エントリポイント
    - .text *.text プログラム
    - .rodata 定数
    - .data 初期値あり変数の初期値
- 0x9000_0000 ~ 0x9000_FFFF : RAM
    - .data 初期値あり変数の実体
    - .bss 初期値なし変数
    - stack (上から)

# エントリポイント (boot.s)
- .dataの初期値をRAMへコピー
- .bssをクリア
- spをセット
