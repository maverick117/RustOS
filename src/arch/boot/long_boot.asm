  global long_start
  extern setup_idt
  extern init_text_console
  extern rust_start
  extern print_char

  section .text
  bits 64
long_start:
  cli                              ; Ensure interrupts are disabled
  xor rax, rax
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  
  ; Need to setup idt

  call setup_idt                  ; Setup IDT
  call init_text_console          ; Setup text console

 ; mov rax, 0x2f472f4e2f4f2f4c      ; Output 'LONG MODE OK'
 ; mov qword [0xb8000], rax
 ; mov rax, 0x2f442f4f2f4d2f20
 ; mov qword [0xb8008], rax
 ; mov rax, 0x2f4b2f4f2f202f45
 ; mov qword [0xb8010], rax

  mov qword rcx, 0x0
  mov qword rbx, [assembly_success_msg_len]
.loop
  xor rax, rax
  mov byte al, [assembly_success_msg + rcx]
  mov byte ah, 0x2f
  mov qword rdi, rax
  call print_char
  inc rcx
  cmp rcx, rbx
  jl .loop


  xor rax, rax
  xor rbx, rbx
  xor rcx, rcx
  xor rdx, rdx
;  call rust_start                 ; Start Rust Kernel Section

  hlt

  section .data
assembly_success_msg:
  db 0x0a, "BOOT GOOD", 0x0a, 0x0a,"Assembly boot successful. Handing control to Rust code...", 0x0a
assembly_success_msg_len:
  dq assembly_success_msg_len - assembly_success_msg