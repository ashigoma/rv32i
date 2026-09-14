# 構文
1引数まで、recあり、tupleなし、パターンマッチなし、文字列操作なし
多相関数なし
```
(int)
(bool)
(string)
(unit)
x

(e)

e1 [+ -] e2 
e1 [< <= > >= == !=] e2
e1 [| &] e2
!e1
if e1 then e2 else e3

let x = e1 in e2

fun x -> e

x e
let rec x = e1 in e2
e1; e2

```

# 中間言語、アセンブリ
使用されるスコープの外で定義されるような変数である自由変数、その参照関係をあらかじめ列挙しておく。
自由変数は実体化時にlambda closure構造体にキャプチャされ、ヒープに置かれる。GCなし。
lambda closure構造体は、実体化した側のスタックフレームへのポインタ、call先のアドレス、自由変数を保持する。
自由変数以外の変数はすべてスタックフレームに置かれる。
関数の呼び出し時は、引数をレジスタ渡し、lambda closure構造体へのポインタをスタック渡し。

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
