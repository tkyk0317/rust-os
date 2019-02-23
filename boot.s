.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    /* Set up stack pointer. */
    lui     sp, %hi(stacks + 4096)
    ori     sp, sp, %lo(stacks + 4096)
    /* Now jump to the rust world; __start_rust.  */
    j       __start_rust

.bss

.global stacks
stacks:
    .skip 4096
