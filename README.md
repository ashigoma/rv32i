# shobo_cpu_jikken

# やること
- RV32Iをsystem verilogで実装、riscv-testsで動作確認 (動けばよい。標準出力はMMIOとする。できるだけQEMUに仕様を合わせる)
- 適当なocamlサブセットを自前で定義、コンパイラをrustで記述、QEMUで動作確認(MMIOで標準出力を叩けるものを使用)
- fizzbuzz, fibを動かす
ここまで