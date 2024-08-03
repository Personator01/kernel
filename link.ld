/* Based on: https://wiki.osdev.org/Bare_Bones#Booting_the_Operating_System */
ENTRY(_start)
SECTIONS {
. = 2M;
.text : ALIGN(4K)
	{
	*(.multiboot)
	*(.text)
	}
.rodata : ALIGN(4K)
	{
	*(.rodata .rodata.* .rodata..*)
	}
.data  : ALIGN(4K)
	{
	*(.data)
	}
.bss  : ALIGN(4K)
	{
	*(COMMON)
	*(.bss)
	}
}
OUTPUT_FORMAT(elf64-x86-64)
/*
STARTUP(boot.o)
*/
OUTPUT(kernel.bin)
