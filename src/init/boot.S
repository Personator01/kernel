# Initial startup sequence for x86_64
# Based on: https://wiki.osdev.org/Bare_Bones#Booting_the_Operating_System

# Multiboot header
.set ALIGN, 	1 << 0
.set MEMINFO, 	1 << 1
.set FLAGS, 	ALIGN | MEMINFO
.set MAGIC, 	0x1BADB002
.set CHECKSUM,	-(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .data
aaaa: 
	.long 4

# Initialize stack to 16KiB
.section .bss
.align 16
stack_bottom:
.space 16384
stack_top:

# Main entry point
.section .text
.extern kern_main
.globl _start
.type _start, @function
_start:
	mov $aaaa, %esp
	call kern_main
	cli
1:	hlt
	jmp 1b

.size _start, . - _start


