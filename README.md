# shobo_cpu_jikken

# やること
- RV32Iをsystem verilogで実装
- 適当なocamlサブセットを自前で定義、コンパイラをrustで記述
- fizzbuzz, fibを動かす

# テスト
## cpu
- 標準入力なし、標準出力はすべてMMIOに統一
- svのほうは、トレースログとMMIO標準出力を別々の場所にリダイレクトできるようにする
- 適当なcプログラムをnostdlibとかつけた状態でgccでコンパイルして実行、トレースログを[emulator](https://github.com/sysprog21/rv32emu)と比較する

## コンパイラ
- print_stringなどは自前で定義
- 適当なocamlテストプログラムをいくつか用意、普通に処理系を通すのとコンパイル→実行で標準出力を比較する

# 作るべきもの
- svで記述した自前RV32I
    - 標準命令のみ
    - メモリは1サイクル読み書き可能として設計
    - MMIO + $write で標準出力に1文字出せる
    - emulatorとフォーマットを合わせたトレースログを自動的にfileに出す
    - cpu例外は $display -> $finish
- スタートアップコード
    - mainへのjump
    - 戻ってきたときのebreak
- 標準出力のサポート
    - 1文字出力 (asm)
    - メモリ上の文字列出力 (c)
    - 整数の出力 (c)
- cpuテストスクリプト
    - バイナリを走らせ、トレースログをsv側と[emulator](https://github.com/sysprog21/rv32emu)とで比較する
    - バイナリは、手書きアセンブリからgccでアセンブル もしくは 適当なcプログラムをnostdlibとかつけた状態でgccでコンパイル で作成
- 自作ocaml subset compiler
    - RV32I ABI準拠
    - GCなし, 部分適用なし, レジスタ割り当てはせずスタックマシン的にやる
- ocaml subset compiler テストプログラム
    - コンパイルして走らせたときの標準出力
