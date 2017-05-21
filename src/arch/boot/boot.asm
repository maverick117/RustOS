	global start
	global multiboot_loc

; gdt.asm
	extern populate_gdt
	extern DATA_SEL
	extern CODE_SEL_32
	extern CODE_SEL_64

; idt.asm
	extern populate_idt

	extern long_start

	extern kernel_size

%include "../util/consts.asm"

	section .text_early
	bits 32

start:
  cli                             ; Clear interrupts

	call check_multiboot            ; Check for multiboot startup
	mov dword [multiboot_loc], ebx  ; Move multiboot information location to mem
	call check_cpuid                ; Check for CPUID restraints
	call check_amd64                ; Check if supports long mode

	call populate_gdt               ; lgdt
	
	mov eax, DATA_SEL
	mov ds, eax
	mov es, eax
	mov fs, eax
	mov gs, eax
	mov ss, eax

	mov esp, (STACK_PAGES_PHYS + S_PAGES * PAGE_SIZE)
	mov ebp, esp

	jmp CODE_SEL_32:gdt_start

gdt_start:
	
	call populate_idt               ; Populate idt
	
	; Size of kernel rounded up to page size
	mov eax, kernel_size
	add eax, PAGE_MASK
	and eax, ~PAGE_MASK

	; end of kernel page
	mov ebx, eax
	add ebx, KERNEL_START

	; pte index of first kernel page
	mov ecx, PTE(KERNEL_START)

	; page structure count
	mov edx, 4

.count_early:

;
;   if (pte_index == 512) reserved_pages++; pte_index = 0; else pte_index++;
;

	sub eax, PAGE_SIZE
	cmp ecx, 512
	jne .no_new_pt

	add edx, 1
	mov ecx, 0
	jmp .ce_loop_end

.no_new_pt:
	add ecx, 1
.ce_loop_end:
	cmp eax, 0
	jmp .count_early

	; ebx = page aligned end kernel address
	; edx = number of page tables needed to fully map kernel
	; ecx = end of kernel paging address

	mov ecx, edx
	shl ecx, PAGE_SHIFT
	add ecx, ebx
	add ecx, PAGE_SIZE

	mov eax, ebx

; Nullify page memory areas for our use
zero_page_mem:
	mov dword [eax], 0x0
	add eax, 4
	cmp eax, ecx
	jne zero_page_mem








; Preliminary checks on system

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


error:
	mov dword [0xb8000], 0x4f524f45 ; Print 'ERR: ' to VGA screen buffer
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov [0xb800a], al
	hlt

	
	section .data

multiboot_loc:
	dq 0x0
