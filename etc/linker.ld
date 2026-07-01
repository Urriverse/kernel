PHDRS
{
    headers PT_PHDR PHDRS;
    text    PT_LOAD FILEHDR PHDRS;
    rodata  PT_LOAD;
    data    PT_LOAD;
    dynamic PT_DYNAMIC;
}

KERNEL_VMA = 0xFFFFFFFFFF000000;

SECTIONS
{
    . = SIZEOF_HEADERS + KERNEL_VMA;
    executable_start = KERNEL_VMA;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.sdata2 .sdata2.*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)

        KEEP(*(.limine_requests_start_marker))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end_marker))
    } :data

    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
    } :data

    executable_end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
        *(.interp)
    }

    executable_len = . - executable_start;
}