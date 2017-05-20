; File for setting up interrupt descriptor table

global setup_idt
global set_isr

section .text
bits 64

; Function for setting up interrupts in long mode

setup_idt:
    
    ret


; Function for setting Interrupt Service Routines for interrupts

set_isr:

    ret


default_isr: ; The default isr, clears maskable interrupts and halts the machine
    cli
    hlt

section .rodata
idt64:
    resb 4096 ; 16 byte per entry * 256 entries
