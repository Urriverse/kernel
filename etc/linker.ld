PHDRS
{
    headers PT_PHDR PHDRS;
    text    PT_LOAD FILEHDR PHDRS;
    rodata  PT_LOAD;
    data    PT_LOAD;
    dynamic PT_DYNAMIC;
}

KERNEL_VMA = 0xFFFFFFFFFF000000;

EXTERN(executable_start executable_len);

SECTIONS
{
    . = SIZEOF_HEADERS + KERNEL_VMA;
    PROVIDE(executable_start = .);

    .text : {
        *(.text .text.*)
    }

    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .data : {
        *(.data .data.*)
        KEEP(*(.limine_requests*))
    }

    .bss : {
        *(.bss .bss.*)
    }

    PROVIDE(executable_len = . - executable_start);

    .metadata : {
        KEEP(*(.metadata))
        LONG(executable_start);
        LONG(executable_len);
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
        *(.interp)
    }
}
