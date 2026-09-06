# ビルド
```
cargo build
```

# パーサの生成
`grammer.rustylr` -> `grammer.rs`
```
cargo install rustylr
~/.cargo/bin/rustylr src/grammer.rustylr src/grammer.rs
```