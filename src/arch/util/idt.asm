    global set_isr_gate
    global set_default_isr

    extern idt64
    extern idt64.pointer
    extern gdt64.code

    extern interrupt_handler

    bits 64
    section .text


%macro SET_ISR 1
mov rdi, %1
mov rsi, isr%1
call set_isr_gate
%endmacro

set_default_isr:

    %assign i 0
    %rep 256
    SET_ISR i
    %assign i (i+1)
    %endrep

    ret

; Function to set one isr gate for one idt entry
; fn set_isr_gate(num : usize, addr: usize) , registers rdi and rsi
set_isr_gate:
    push rbx
    mov rbx, rdi
    shl rbx, 4 ; Get the byte offset to the entry
    mov rax, idt64
    add rax, rbx ; Get the absolute offset
    mov rbx, rsi ; Move the address of the isr to rbx
    mov word [rax], bx ; First part of entry, offset [0:15]
    add rax, 2

    mov rcx, gdt64.code
    mov word [rax], cx ; Segment selector
    add rax, 2

    mov byte [rax], 0  ; IST
    inc rax

    mov byte [rax], (1 << 7) | (0 << 5) | 0xe
    inc rax

    shr rbx, 16
    mov word [rax], bx ; Second part of offset
    add rax, 2

    shr rbx, 16
    mov dword [rax], ebx; Last part
    add rax, 4

    mov dword [eax], 0

    pop rbx
    ret

common_interrupt_stub:
    pop qword rsi
    pop qword rdi

    call interrupt_handler

    
    iretq

%macro GEN_ISR 1
    global isr%1
isr%1:
    cli
    push qword 0
    push qword %1
    jmp common_interrupt_stub
%endmacro

%assign i 0
%rep 256
GEN_ISR i
%assign i (i+1)
%endrep