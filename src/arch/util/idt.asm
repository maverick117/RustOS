%include "consts.asm"

bits 32
SECTION .text_early

EXTERN CODE_SEL_64



%define FLAG_INTERRUPT     0xe

; Permission rings
%define FLAG_R0            (0 << 5) 
%define FLAG_P             (1 << 7)

%define IDT_ENTRIES        256

; Macro to generate isr

%macro ISR 1
isr%1:
    jmp $
%endmacro

ISRS:
%assign i 0
%rep IDT_ENTRIES
ISR i
%assign i (i+1)
%endrep

%define ISR_SIZE (isr1 - isr0)

ALIGN 8
IDTR:
    dw (IDTEND - IDT - 1)
    dq IDT

%macro IDTENTRY 0
    dd 0x00000000
    dd 0x00000000
    dd 0x00000000
    dd 0x00000000
%endmacro

ALIGN 8
IDT:
%assign i 0
%rep IDT_ENTRIES
IDTENTRY
%assign i (i+1)
%endrep
IDTEND:

GLOBAL populate_idt
populate_idt:
    mov eax, IDT
    mov ebx, isr0
    or ebx, (VIRT_BASE & 0xFFFFFFFF)

.idt_init_one:
    mov ecx, ebx
    mov word [eax], cx ; First entry, offset[0-15], word
    add eax, 2

    mov word [eax], CODE_SEL_64 ; Second entry, selection, word
    add eax, 2
    
    mov byte [eax], 0           ; ist, byte
    add eax, 1

    mov byte [eax], (FLAG_P | FLAG_R0 | FLAG_INTERRUPT) ; Flags, byte
    add eax, 1

    shr ecx, 16
    mov word [eax], cx ; offset[31-16], word
    add eax, 2

    mov dword [eax], (VIRT_BASE >> 32) ; Long mode
    add eax, 4

    mov dword [eax], 0x0
    add eax, 4

    add ebx, ISR_SIZE

    cmp eax, IDTEND
    jl .idt_init_one

    lidt[IDTR]
    ret

    BITS 64
    GLOBAL fixup_idtr
fixup_idtr:
    mov rax, VIRT_BASE + IDTR + 2
    mov rbx, VIRT_BASE + IDT
    mov qword [rax], rbx

    sub rax, 2
    lidt[rax]
    ret