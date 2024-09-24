
    .text
_start: .global _start
    mov r1, #34
    mov r2, #35

    add r0, r1, r2

    mov r0, #0
    mov r1, #0
    mov r2, #0

init_vtor:
    ldr r4, =__isr_vector
    ldr r5, =__vtor_addr

    str r4, [r5]

trigger_exception:
    blx r1 // blx currently triggers an exception inside our emulator, this is only to test exceptions

    ldr r1, =_start
    bx r1


UsageFault_Handler:
    mov r0, #24

    ldr r2, =UsageFault_Handler
    bx r2


__vtor_addr:
    .long   3758157064

__isr_vector:
    .long   __StackTop
    .long   Reset_Handler
    .long   NMI_Handler
    .long   HardFault_Handler
    .long   MemoryManagement_Handler
    .long   BusFault_Handler
    .long   UsageFault_Handler
    .long   0
    .long   0
    .long   0
    .long   0
    .long   SVC_Handler
    .long   DebugMon_Handler
    .long   0
    .long   PendSV_Handler
    .long   SysTick_Handler


