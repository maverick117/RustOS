	global start
	global multiboot_loc
	extern long_start

	global gdt64
	global gdt64.code
	global idt64


; Macro for generating an idt
%macro GEN_IDT 0
%rep 256
dq 0x0
dq 0x0
%endrep
%endmacro

	section .text
	bits 32
start:
  cli                             ; Clear interrupts
	mov esp, stack_top              ; Move stack to top
	call check_multiboot            ; Check for multiboot startup
	mov dword [multiboot_loc], ebx        ; Move multiboot information location to mem
	call check_cpuid                ; Check for CPUID restraints
	call check_amd64                ; Check if supports long mode
	call set_page_table             ; Prepare page table for long mode
	call enable_paging
	mov dword [0xb8000], 0x2f4b2f4f ; Green 'OK' to VGA screen buffer
	lgdt [gdt64.pointer]
	lidt [idt64.pointer]
	jmp gdt64.code:long_start       ; Long jump to switch to long mode
	mov al, 'K'
	jmp error

check_multiboot:
	cmp eax, 0x36d76289             ; Check for magic value
	jne .invalid_boot
	ret
.invalid_boot:
	mov al, '1'
	jmp error

check_cpuid:
	pushfd
	pop eax                         ; Push cpu flags onto stack and pop to eax
	mov ecx, eax
	xor eax, 1<<21                  ; Flip id bit
	push eax
	popfd
	pushfd
	pop eax
	cmp eax, ecx
	je .no_cpuid
	ret
.no_cpuid:
	mov al, '2'
	jmp error

check_amd64:
  mov eax, 0x80000000
	cpuid
	test edx, 1<<29
	jz .i386
	ret
.i386:
  mov al, '3'
	jmp error

set_page_table:
	mov eax, p3_table               ; Move p3_table addresss to eax
	or eax, 0b11                    ; present+writable
	mov [p4_table], eax             ; p3_table as first entry of p4_table
	mov eax, p2_table               ; Move p2_table address to eax
	or eax, 0b11                    ; p+w
	mov [p3_table], eax             ; p2_table as first entry of p3_table
  mov ecx, 0                      ; Counter
.set_p2:
	mov eax, 0x200000
	mul ecx
	or eax, 0b10000011              ; Huge + present + writable
	mov [p2_table + ecx * 8], eax
	inc ecx
	cmp ecx, 512
	jne .set_p2
	ret

enable_paging:
	mov eax, p4_table
	mov cr3, eax

	mov eax, cr4
	or eax, 1<<5
	mov cr4, eax

	mov ecx, 0xC0000080
	rdmsr
	or eax, 1<<8
	wrmsr

	mov eax, cr0
	or eax, 1<<31
	mov cr0, eax
	ret

error:
	mov dword [0xb8000], 0x4f524f45 ; Print 'ERR: ' to VGA screen buffer
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov [0xb800a], al
	hlt


	section .bss ; Stack
	align 4096
p4_table:
	resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096
stack_bottom:
  resb 4096
stack_top:

	section .rodata
gdt64:
	dq 0x0
.code: equ $ - gdt64
 	dq (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

idt64:
GEN_IDT
.pointer:
	dw $ - idt64 - 1
	dq idt64


	section .data

multiboot_loc:
	dq 0x0
