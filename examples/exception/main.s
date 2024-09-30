    .syntax unified

    .section .text

    .global vtable
    .global reset_handler

vtable:
    .word _estack
    .word reset_handler
    .size vtable, .-vtable

reset_handler:
    ldr r4, =apa2
    ldr r5, =apa

    ldr r5, [r5]

done:
    b done

    .section .data

apa:    .word 69
apa3:   .word 61
apa4:   .word 62
apa5:   .word 63
apa2:   .word 68


