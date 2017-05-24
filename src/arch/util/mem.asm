; Assembly utilities for memory management


global flush_tlb

    section .text
    bits 64
flush_tlb: ; fn flush_tlb(addr: *const _);
    push rbx
    
    mov rax, rsi
    invlpg [rsi]


    pop rbx
    ret