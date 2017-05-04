section .multiboot_header
header_start:
  dd 0xe85250d6 ; Magic number
  dd 0          ; Protected Mode
  dd header_end - header_start ; length
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
  dd 0
  dd 0
  dd 8
header_end:
