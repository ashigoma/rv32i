# ビルドと実行

```
make
spike --isa=rv32i --log-commits ./build/hello.elf 2> spike_trace.log
```

# cpu側の特殊仕様まとめ
- 疑似標準出力
    - 0x10000000 へのstoreで、sv側から標準出力に1文字書き出し (spike標準)
    - 0x10000004 へのstoreで、sv側から標準出力に16進整数を書き出し
    - 0x10000008 へのstoreで、sv側から標準出力に整数を書き出し
- 実行開始
    - 0x80000000 に entry pointがおいてある。そこから実行開始する
- 実行停止
    - cpu側では、ebreak命令
    - sim側では、特定アドレスへのstore