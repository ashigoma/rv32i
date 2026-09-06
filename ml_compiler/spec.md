# 構文
1引数まで、recあり、tupleなし、パターンマッチなし、文字列操作なし
```
(int)
(bool)
(string)
(unit)

x
e1 e2
e1 [+ -] e2 
e1 [< <= > >= == !=] e2
e1 [| &] e2
!e1
if e1 then e2 else e3
let x = e1 in e2
let rec x = e1 in e2
fun x -> e
(e)
```

# 中間言語、アセンブリ
使用されるスコープの外で定義されるような変数である自由変数をあらかじめ列挙しておく。
&によるlambda closure作成時に環境構造体(自由変数の数だけフィールドを持つ)にキャプチャされ、ヒープに置かれる。GCなし。
それ以外の変数はスタックフレームに置かれる。関数の引数も呼び出し時にスタックフレームに置かれる。レジスタ割り当てなし。
すべての関数は環境ポインタ + 1引数をそれぞれレジスタ経由で受け取り、値を1つレジスタ経由で返す。引数名は関数名のとなりにメモしておく。2引数目以降は自由変数とみなす。
環境は呼び出し時に関数に渡されたものを関数内で操作する。
_mainには空の環境とダミー引数が渡される。

```ocaml
let y = 2 in
let f = fun x -> x + y in
f 3
```

```
_main _dummy:
    y = 2
    f = &_f1
    _x2 = app f 3
    _x2

_f1 x y:
    _x1 = add x y
    _x1
```

```ocaml
let fib = fun n -> if n <= 1 then 1 else fib (n-2) + fib (n-1) in fib 10
```

```
_main _dummy:
    fib = &_f1
    _x7 = app fib 10
    _x7

_f1 n fib:
    _x1 = leq n 1
    b _x1 _l1
    _x3 = sub n 2
    _x4 = app fib _x3
    _x5 = sub n 1
    _x6 = app fib _x5
    _x2 = add _x4 _x6
    j _l2
_l1:
    _x2 = 1
_l2:
    _x2
```