# rv32i
- rv32iのsystem verilog実装
    - [仕様など](rv32i/spec.md)
- OCaml subset の rv32i compiler
    - [仕様など](ml_compiler/spec.md)

## build & run
### cpuとコンパイラをビルド
```
make
```

### OCamlコードをrv32i向けにビルドして実行
`src/1p1.ml`なら
```
make run 1p1
```