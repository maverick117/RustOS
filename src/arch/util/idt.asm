; File for setting up interrupt descriptor table

    global setup_idt
    global set_isr

    extern gdt64.code


; Define some constants

FLAG_INTERRUPT equ 0xe

FLAG_R0          equ (0 << 5) 
FLAG_P           equ (1 << 7)

IDT_ENTRIES      equ 256

IDT_SIZE         equ (idt64.end - idt64)

DEFAULT_IST_SIZE equ (default_isr.end - default_isr)

%macro IDT_ENTRY 0
    dq 0x0000000000000000
    dq 0x0000000000000000
%endmacro

    section .text
    bits 64

default_isr: ; The default isr, clears maskable interrupts and halts the machine
    cli
    ;mov rax, 0x4f204f524f524f45 ; Print "ERR " to the screen
    ;mov qword [0xb8000], rax
    hlt
.end:

; Function for setting up interrupts in long mode

setup_idt:
    push rbx 



.set_one_entry:




    push rcx ; Preserve rcx counter
    call set_isr
    pop rcx

    inc rcx
    cmp rcx, 256
    jl .set_one_entry

    pop rbx
    ret


; Function for setting Interrupt Service Routines for interrupts

set_isr:
    ; As per calling convention, rdi rsi rdx rcx r8 are used for argument passing
    ; rdi: interrupt number
    ; rsi: offset
    ; rdx: selector
    ; rcx: ist
    ; r8 : type_attr

    

    ret




    section .rodata
    align 8
idt64:
    %assign i 0
    %rep IDT_ENTRIES
    IDT_ENTRY
    %assign i (i+1)
    %endrep
.end:

IDTR:
    dw (idt64.end - idt64 - 1)
    dq idt64