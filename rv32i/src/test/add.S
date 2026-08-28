.section .text
.global main

main:
    addi    sp, sp, -16
    sw      ra, 12(sp)


    li      t0, 15
    li      t1, 27
    add     a0, t0, t1
    call    print_int
    li      a0, 10
    call    print_char
    
    
    lw      ra, 12(sp)
    addi    sp, sp, 16
    ret