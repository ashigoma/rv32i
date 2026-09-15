# 構文
1引数まで、recあり
tupleなし、パターンマッチなし、文字列操作なし、リストなし
相互再帰なし、多相関数なし
```
42
true
"hi"
()
x

(e)

e1 [+ -] e2 
e1 [< <= > >= == !=] e2
e1 [| &] e2
!e1
if e1 then e2 else e3
e1; e2

fun x -> e
x e

let x = e1 in e2
let rec x = e1 in e2

print_string s
print_int n
print_bool b
```

# 型検査
多相型、primitive型の不一致、自己言及をコンパイルエラーにする

# 中間言語
```
_main _dummy:
        f = &_f1
        g = &_f2
        _x7 = 1
        _res = g _x7
        ret _res
_f1 x:
        _x2 = x
        _x3 = 2
        _x1 = _x2 + _x3
        ret _x1
_f2 y:
        _x6 = y
        _x5 = f _x6
        _x4 = f _x5
        ret _x4
```

# 変数解析
関数の外で定義された後関数内で使われるような変数が自由変数、そうでない変数は束縛変数。
各関数に対して、以下を静的に決定する。
- 自由変数→束縛変数の参照関係(何回static linkをたどるべきか)
    - 同名の自由変数が複数あれば直近の定義を採用
    - 未定義の自由変数があればコンパイルエラーにする
- 束縛変数→スタックフレーム内での束縛変数のindex

# アセンブリ
- 引数のレジスタ渡しはしない
- spはcallerが減算→関数呼び出し→callerが加算
- stack frameの構造
    - calleeに対応する lambda closureへのポインタ
    - 束縛変数(引数はこれの0番目とみなす)
- heap
    - lambda closure
    - 文字列
- lambda closureの構造
    - 関数の開始アドレス
    - 自身が作られたときのspの値
    - キャプチャした自由変数たちの値

GCはたぶん実装しない
