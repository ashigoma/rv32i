.section .text
.global main

main:
    addi    sp, sp, -16
    sw      ra, 12(sp)

    lui     a0, 0xfffff
    auipc   a1, 0

    jal     t0, label_jal_target
    nop
label_jal_target:
    auipc   t1, 0
    addi    t1, t1, 12
    jalr    t2, 0(t1)
    nop
label_jalr_target:

    addi    t0, zero, -1
    addi    t1, zero, 1

    beq     t0, t0, label_beq
    nop
label_beq:

    bne     t0, t1, label_bne
    nop
label_bne:

    blt     t0, t1, label_blt
    nop
label_blt:

    bge     t1, t0, label_bge
    nop
label_bge:

    bltu    t1, t0, label_bltu
    nop
label_bltu:

    bgeu    t0, t1, label_bgeu
    nop
label_bgeu:

    addi    t0, zero, -1
    sw      t0, 0(sp)

    lb      t1, 0(sp)
    lbu     t2, 0(sp)
    lh      t3, 0(sp)
    lhu     t4, 0(sp)
    lw      t5, 0(sp)

    sb      t2, 4(sp)
    sh      t4, 6(sp)
    sw      t5, 8(sp)

    addi    a0, zero, -1
    slli    a1, a0, 31
    srli    a2, a0, 1
    srai    a3, a0, 1

    addi    a4, zero, 33
    sll     a5, a0, a4
    srl     a6, a0, a4
    sra     a7, a0, a4

    addi    t0, zero, -1
    slti    t1, t0, 0
    sltiu   t2, t0, 1
    sltiu   t3, zero, 1
    xori    t4, t0, -1
    ori     t5, zero, -1
    andi    t6, t0, 0x7ff

    lui     s0, 0x80000
    addi    s1, zero, 1

    add     s2, s0, t0
    sub     s3, s0, s1
    slt     s4, s0, s1
    sltu    s5, s0, s1
    xor     s6, s0, t0
    or      s7, s0, s1
    and     s8, s0, t0

    lw      ra, 12(sp)
    addi    sp, sp, 16
    ret