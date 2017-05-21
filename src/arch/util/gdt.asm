%include "consts.asm"

bits 32
section .text_early

; GDT assembly

;/* Flag declaration */

%define FLAG_CODE        0xa
%define FLAG_DATA        0x2

%define FLAG_USER        (1 << 4)
%define FLAG_SYSTEM      (0 << 4)

; Permission Ring
%define FLAG_R0          (0 << 5)
%define FLAG_R1          (1 << 5)
%define FLAG_R2          (2 << 5)
%define FLAG_R3          (3 << 5)

; Present
%define FLAG_P           (1 << 7) 

; 32-bit compatibility
%define FLAG_32          (1 << 14)

; 4k page granularity
%define FLAG_4k          (1 << 15)

%define FLAGS_COMMON_32  (FLAG_USER | FLAG_R0 | FLAG_P | FLAG_32 | FLAG_4k)
%define FLAGS_CODE_32    (FLAG_CODE | FLAGS_COMMON_32)
%define FLAGS_DATA_32    (FLAG_DATA | FLAGS_COMMON_32)

; Bit 13 is the "long" bit 
%define FLAG_L           (1 << 13)

%define FLAGS_CODE_64    (FLAG_USER | FLAG_R0 | FLAG_P | FLAG_L | FLAG_4k | FLAG_CODE)

;/* 1 = flags, 2 = base, 3 = limit */
%macro GDTENTRY 3
    dw ((%3) & 0xFFFF)
    dw ((%2) & 0xFFFF)
    db (((%2) & 0xFF0000) >> 16)
    dw ((%1) | (((%3) & 0xF000) >> 8))
    db (((%2) & 0xFF000000) >> 24)
%endmacro

GLOBAL CODE_SEL_32
GLOBAL DATA_SEL
GLOBAL CODE_SEL_64

ALIGN 8
GDT:
    dq 0x0 ; Null Entry
CODE_SEL_32 equ $ - GDT
    GDTENTRY FLAGS_CODE_32, 0x0, 0xFFFFF
DATA_SEL equ $ - GDT
    GDTENTRY FLAGS_DATA_32, 0x0, 0xFFFFF
CODE_SEL_64 equ $ - GDT
    GDTENTRY FLAGS_CODE_64, 0x0, 0xFFFFF
GDTEND:

ALIGN 8
GDTR:
    dw (GDTEND - GDT - 1)
    dq GDT

GLOBAL populate_gdt

populate_gdt:
    lgdt[GDTR]
    ret

bits 64
GLOBAL fixup_gdtr
fixup_gdtr:
    mov rax, VIRT_BASE + GDTR + 2
    mov rbx, VIRT_BASE + GDT
    
    mov qword [rax], rbx
    sub rax, 2
    lgdt[rax]

    ret