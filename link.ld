/* Based on: https://wiki.osdev.org/Bare_Bones#Booting_the_Operating_System */
ENTRY(_start)
SECTIONS {
= 2M;
.text BLOCK(4K) : ALIGN(4K)
	{
	*(.multiboot)
	*(.text)
	}
.rodata BLOCK(4K): ALIGN(4K)
	{
	*(.rodata)
	}
.data BLOCK(4K): ALIGN(4K)
	{
	*(.data)
	}
.bss BLOCK(4K): ALIGN(4K)
	{
	*(COMMON)
	*(.bss)
	}
OUTPUT_FORMAT(elf64-x86-64)
STARTUP(preinit.o)
OUTPUT(kernel.bin)
