  global long_start

  section .text
  bits 64
long_start:
  cli                              ; Ensure interrupts are disabled

  hlt
