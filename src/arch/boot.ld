OUTPUT_FORMAT("elf64-x86-64")
ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		KEEP(*(.multiboot_header))
	}

	.text_early :
	{
		*(.text_early)
	}

	kernel_start = 0xFFFFFFFF8010000;

	.text 0xFFFFFFFF80102000 : AT(0x102000)
	{
		*(.text)
	}

	.data :
	{
		*(.data .data.*)
	}

	.bss :
	{
		*(.bss .bss.*)
	}

	.rodata :
	{
		*(.rodata .rodata.*)
	}

	.data.rel.ro :
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	}

	kernel_end = . ;
	kernel_size = kernel_end - kernel_start;

}
