.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

.equ STACK_SIZE, 4096
_start:
    /* Set up stack pointer. */
    lui     sp, %hi(stacks + STACK_SIZE)
    ori sp, sp, %lo(stacks + STACK_SIZE)
    /* Now jump to the rust world; __start_rust.  */
    j __start_rust

_dummy:
    j interrupt /* dummy call because compiler remove no-used function */

.bss

.global stacks
.align 2
stacks:
  .skip STACK_SIZE


