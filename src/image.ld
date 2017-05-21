OUTPUT_FORMAT("elf64-x86-64")
ENTRY(start)

SECTIONS {
	. = 1M;

	.boot : 
	{
		KEEP(*(.boot))
	}

	.text :
	{
		*(.text .text.*)
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
