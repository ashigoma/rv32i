.global _main
_main:
# f = &_f1
        lw x5, 8(x2)
        sw x5, 0(x3)
        sw x2, 4(x3)
        li x5, 32
        sw x5, 8(x3)
        la x5, _f1
        sw x5, 12(x3)
        sw x3, 16(x2)
        addi x3, x3, 16
# _x4 = f
        lw x5, 16(x2)
        sw x5, 20(x2)
# _x5 = 2
        li x5, 2
        sw x5, 24(x2)
# _res = _x4 _x5
        lw x5, 20(x2)
        lw x6, 8(x5)
        mv x7, x2
        lw x28, 24(x2)
        sub x2, x2, x6
        sw x7, 0(x2)
        sw x1, 4(x2)
        sw x5, 8(x2)
        sw x28, 12(x2)
        lw x5, 12(x5)
        jalr x5
        lw x1, 4(x2)
        lw x2, 0(x2)
        sw x10, 28(x2)
# ret _res
        lw x10, 28(x2)
        ret
_f1:
# _x2 = x
        lw x5, 12(x2)
        sw x5, 16(x2)
# _x3 = 1
        li x5, 1
        sw x5, 20(x2)
# _x1 = _x2 + _x3
        lw x6, 16(x2)
        lw x7, 20(x2)
        add x5, x6, x7
        sw x5, 24(x2)
# ret _x1
        lw x10, 24(x2)
        ret
