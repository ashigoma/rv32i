.section .text.init
.global _start
.global print_string
.global print_int

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

print_string:
    lw t0, 12(sp)
loop:
    lbu t1, 0(t0)
    beq t1, zero, end
    addi t0, t0, 1
    li t2, 0x20000000
    sb t1, 0(t2)
    j loop
end:
    ret

print_int:
    lw t0, 12(sp)
    li t1, 0x20000008
    sw t0, 0(t1)
    ret

# spikeを終了するためのMMIO
.section .tohost, "aw", @progbits
.align 4
.global tohost
tohost:   .dword 0
.align 4
.global fromhost
fromhost: .dword 0
