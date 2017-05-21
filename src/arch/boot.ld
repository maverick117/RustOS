ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text)
	}

	.data : {
		*(.data .data.*)
	}

	.bss : {
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
}
