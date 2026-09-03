.section .text
.global main

main:
    addi    sp, sp, -16
    sw      ra, 12(sp)

    lui     t0, 0x12345
    auipc   t1, 0
    jal     ra, label_jal

    beq     t0, t1, l1
l1:
    bne     t0, t1, l2
l2:
    blt     t0, t1, l3
l3:
    bge     t0, t1, l4
l4:
    bltu    t0, t1, l5
l5:
    bgeu    t0, t1, label_branch

label_branch:
    lb      t3, 0(sp)
    lh      t4, 2(sp)
    lw      t5, 4(sp)
    lbu     t6, 0(sp)
    lhu     a0, 2(sp)

    sb      t3, 0(sp)
    sh      t4, 2(sp)
    sw      t5, 4(sp)

    addi    a1, a0, 1
    slti    a2, a0, 10
    sltiu   a3, a0, 10
    xori    a4, a0, 0xff
    ori     a5, a0, 0x0f
    andi    a6, a0, 0xf0

    addi    a0, zero, -1
    slli    a7, a0, 2
    srli    t0, a0, 2
    srai    t1, a0, 2

    add     t2, a0, a1
    sub     t3, a0, a1
    sll     t4, a0, a1
    slt     t5, a0, a1
    sltu    t6, a0, a1
    
    xor     a2, a0, a1
    j end
    srl     a3, a0, a1
    sra     a4, a0, a1
    or      a5, a0, a1
    and     a6, a0, a1

end:
    lw      ra, 12(sp)
    addi    sp, sp, 16
    ret

label_jal:
    jalr    t2, 0(ra)