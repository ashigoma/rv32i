.section .text
.global main

main:
    addi    sp, sp, -16
    sw      ra, 12(sp)
    
    lw      ra, 12(sp)
    addi    sp, sp, 16
    ret