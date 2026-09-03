.section .text.init
.global _start

_start:
    # .dataのコピー
    la t0, _sidata
    la t1, _sdata
    la t2, _edata
1:
    bge t1, t2, 2f
    lw  t3, 0(t0)
    sw  t3, 0(t1)
    addi t0, t0, 4
    addi t1, t1, 4
    j   1b
2:
    
    # .bssのクリア
    # verilog側で0初期化しているのでパス

    # spの初期化
    li sp, 0x90010000

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