.section .text.init
.global _start

_start:
    # spの初期化
    li sp, 0x80010000

    # mainを呼ぶ
    call main
    
    # 終わり。spikeではtohostに書き込んで通知
    la t0, tohost
    li t1, 1
    sw t1, 0(t0)

    # svでは$finishを呼ぶ
    ebreak

# spikeを終了するためのなにか
.section .tohost, "aw", @progbits
.align 6
.global tohost
tohost:   .dword 0
.align 6
.global fromhost
fromhost: .dword 0