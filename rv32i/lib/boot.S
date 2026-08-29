.section .text.init
.global _start

_start:
    # .dataのコピー
    # まだ
    
    # .bssのクリア
    # まだ

    # spの初期化
    li sp, 0xa0000000

    # 走らす
    call main
    
    # 終わり。spikeではtohostに書き込んで通知。svでは$finishを呼ぶ
    la t0, tohost
    li t1, 1
    sw t1, 0(t0)
    ebreak

# spikeを終了するためのMMIO
.section .tohost, "aw", @progbits
.align 4
.global tohost
tohost:   .dword 0
.align 4
.global fromhost
fromhost: .dword 0