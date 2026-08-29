
# [RV32I Base Integer Instruction Set](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html)
[アセンブル](http://five-embeddev.com/riscv-user-isa-manual/Priv-v1.12/instr-table.html)
|命令|例|操作|備考|
|---|---|---|---|
|LUI|`lui x1, 0x12345`|`x1` ← `0x12345000`|上位20bitに格納|
|AUIPC|`auipc x1, 0x12345`|`x1` ← `PC + (0x12345 << 12)`|PC相対で上位20bit加算|
|JAL|`jal x1, label`|`x1` ← `PC + 4`, `PC` ← `label`|戻り先を保存して相対ジャンプ|
|JALR|`jalr x1, 4(x2)`|`x1` ← `PC + 4`, `PC` ← `x2 + 4`|戻り先を保存して絶対ジャンプ|
|BEQ|`beq x1, x2, label`|if (`x1 == x2`) `PC` ← `label`||
|BNE|`bne x1, x2, label`|if (`x1 != x2`) `PC` ← `label`||
|BLT|`blt x1, x2, label`|if (`x1 < x2` `[signed]`) `PC` ← `label`||
|BGE|`bge x1, x2, label`|if (`x1 >= x2` `[signed]`) `PC` ← `label`||
|BLTU|`bltu x1, x2, label`|if (`x1 < x2` `[unsigned]`) `PC` ← `label`||
|BGEU|`bgeu x1, x2, label`|if (`x1 >= x2` `[unsigned]`) `PC` ← `label`||
|LB|`lb x1, 4(x2)`|`x1` ← `SignExt(M[x2 + 4][7:0])`|1バイト符号拡張ロード|
|LH|`lh x1, 4(x2)`|`x1` ← `SignExt(M[x2 + 4][15:0])`|2バイト符号拡張ロード|
|LW|`lw x1, 4(x2)`|`x1` ← `M[x2 + 4][31:0]`|4バイトロード|
|LBU|`lbu x1, 4(x2)`|`x1` ← `ZeroExt(M[x2 + 4][7:0])`|1バイトゼロ拡張ロード|
|LHU|`lhu x1, 4(x2)`|`x1` ← `ZeroExt(M[x2 + 4][15:0])`|2バイトゼロ拡張ロード|
|SB|`sb x1, 4(x2)`|`M[x2 + 4][7:0]` ← `x1[7:0]`|1バイトストア|
|SH|`sh x1, 4(x2)`|`M[x2 + 4][15:0]` ← `x1[15:0]`|2バイトストア|
|SW|`sw x1, 4(x2)`|`M[x2 + 4][31:0]` ← `x1[31:0]`|4バイトストア|
|ADDI|`addi x1, x2, 1`|`x1` ← `x2 + 1`||
|SLTI|`slti x1, x2, 1`|`x1` ← (`x2 < 1` `[signed]`) ? `1` : `0`||
|SLTIU|`sltiu x1, x2, 1`|`x1` ← (`x2 < 1` `[unsigned]`) ? `1` : `0`||
|XORI|`xori x1, x2, 1`|`x1` ← `x2 ^ 1`||
|ORI|`ori x1, x2, 1`|`x1` ← `x2` \| `1`||
|ANDI|`andi x1, x2, 1`|`x1` ← `x2 & 1`||
|SLLI|`slli x1, x2, 1`|`x1` ← `x2 << 1`|論理左シフト|
|SRLI|`srli x1, x2, 1`|`x1` ← `x2 >> 1`|論理右シフト|
|SRAI|`srai x1, x2, 1`|`x1` ← `x2 >> 1`|算術右シフト|
|ADD|`add x1, x2, x3`|`x1` ← `x2 + x3`||
|SUB|`sub x1, x2, x3`|`x1` ← `x2 - x3`||
|SLL|`sll x1, x2, x3`|`x1` ← `x2 << x3[4:0]`|論理左シフト|
|SLT|`slt x1, x2, x3`|`x1` ← (`x2 < x3` `[signed]`) ? `1` : `0`||
|SLTU|`sltu x1, x2, x3`|`x1` ← (`x2 < x3` `[unsigned]`) ? `1` : `0`||
|XOR|`xor x1, x2, x3`|`x1` ← `x2 ^ x3`||
|SRL|`srl x1, x2, x3`|`x1` ← `x2 >> x3[4:0]`|論理右シフト|
|SRA|`sra x1, x2, x3`|`x1` ← `x2 >> x3[4:0]`|算術右シフト|
|OR|`or x1, x2, x3`|`x1` ← `x2` \| `x3`||
|AND|`and x1, x2, x3`|`x1` ← `x2 & x3`||
|FENCE|`fence iorw, iorw`|-|メモリ・IO順序制御|
|FENCE.TSO|`fence.tso`|-|TSOモデル用メモリフェンス|
|PAUSE|`pause`|-|パイプライン一時停止ヒント|
|ECALL|`ecall`|-|システムコール発生|
|EBREAK|`ebreak`|-|ブレークポイント発生|


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

# エントリポイント (boot.s)
- .dataの初期値をRAMへコピー
- .bssをクリア
- spをセット
