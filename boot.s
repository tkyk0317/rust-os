.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

.equ STACK_SIZE, 4096
_start:
    /* setting mtvec */
    lla t0, _interrupt
    csrw mtvec, t0

    /* Set up stack pointer. */
    lui     sp, %hi(stacks + STACK_SIZE)
    ori sp, sp, %lo(stacks + STACK_SIZE)

    /* Now jump to the rust world; __start_rust.  */
    j __start_rust

.align 4
_interrupt:
    csrr a0, mcause
    j __interrupt
    mret

.bss

.global stacks
.align 4
stacks:
  .skip STACK_SIZE

