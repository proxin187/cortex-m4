    .syntax unified

    .section .text

    .global vtable
    .global reset_handler

vtable:
    .word _estack
    .word reset_handler
    .size vtable, .-vtable

reset_handler:
    ldr r4, =apa

    // ldr r1, =_flash_dstart

done:
    b done

    .section .data

apa:    .word 0xFEEBDAED

/*
    .text
_start: .global _start
    // load the variable into memory
    ldr     r0, =.LCPI0_0
    // str     r0, [sp]

    // write to the variable
    // ldr     r1, [sp]
    // mov     r0, #69
    // str     r0, [r1]

    // store into our variable on the stack, we ignore this
    // str     r0, [sp]

    // 69 is 0x45 in hex
.LCPI0_0:
    .long 69

    .text
_start: .global _start
    mov r1, #34
    mov r2, #35

    add r0, r1, r2

    mov r0, #0
    mov r1, #0
    mov r2, #0

init_vtor:
    // ldr r0, .LCPI0_0
    // str r0, [sp]
    // ldr r1, [sp]
    // mov r0, #69
    // str r0, [r1]

    // NOTE: this may be wrong, looks right
    ldr r4, =__isr_vector
    ldr r5, =vtor_addr

    ldr r6, =0x12345678

    str r4, [r5]

trigger_exception:
    blx r1 // blx currently triggers an exception inside our emulator, this is only to test exceptions

    ldr r1, =_start
    bx r1


UsageFault_Handler:
    mov r0, #24

    ldr r2, =UsageFault_Handler
    bx r2

vtor_addr:
    .long   420
    // .long   3758157064

__isr_vector:
    .long   0 // __StackTop
    .long   0 // Reset_Handler
    .long   0 // NMI_Handler
    .long   0 // HardFault_Handler
    .long   0 // MemoryManagement_Handler
    .long   0 // BusFault_Handler
    .long   UsageFault_Handler
    .long   0
    .long   0
    .long   0
    .long   0
    .long   0 // SVC_Handler
    .long   0 // DebugMon_Handler
    .long   0
    .long   0 // PendSV_Handler
    .long   0 // SysTick_Handler
*/


