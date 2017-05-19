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
