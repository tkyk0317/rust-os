ENTRY(_start)

SECTIONS
{
    /* sifive_uのエントリーポイントアドレス */
    . = 0x80000000;

    .text :
    {
        *(.boot)
        . = 0x80000100;
        *(.vector_table)
        *(.text .text.*)
    }

    /* rodata: Read-only data */
    .rodata :
    {
        *(.rdata .rodata .rodata.*)
    }

    /* data: Writable data */
    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        *(.bss bss.*)
    }
}
