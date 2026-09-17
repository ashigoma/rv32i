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
`src/hello_world.ml`なら
```
make hello_world
```

### rv32i spike simと自前実装rv32iでトレースログが一致しているか確認
```
make compare hello_world
```