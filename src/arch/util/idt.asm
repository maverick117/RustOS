    global set_isr_gate
    global set_default_isr
    global invoke_breakpoint

    extern idt64
    extern idt64.pointer
    extern gdt64.code

    extern interrupt_handler

    bits 64
    section .text
    align 8

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

invoke_breakpoint:

    int 3
    ret

common_interrupt_stub:
    ;mov dword [0xb8000], 0x2f4b2f4f 
    ;push rax
    ;push rcx
    ;push rdx
    ;push rsi
    ;push rdi
    ;push r8
    ;push r9
    ;push r10
    ;push r11

    mov rdi, rsp
    sub rsp, 8
    call interrupt_handler
    ;add rdi, 9*8
    
    ;call interrupt_handler

    ;pop r11
    ;pop r10
    ;pop r9
    ;pop r8
    ;pop rdi
    ;pop rsi
    ;pop rdx
    ;pop rcx
    ;pop rax

    ;mov dword [0xb8004], 0x4f524f45
    ;sti
    iretq

%macro GEN_ISR 1
    global isr%1
isr%1:
    ;cli
    ;push word 0
    ;push qword %1
    jmp common_interrupt_stub
%endmacro

%assign i 0
%rep 256
GEN_ISR i
%assign i (i+1)
%endrep