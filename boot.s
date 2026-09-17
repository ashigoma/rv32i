.section .text.init
.global _start

_start:
    # stack pointerの初期化
    la sp, _stack_top
    # heap pointerの初期化
    la x3, _heap_bottom

    # 走らす
    call _main
    
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
