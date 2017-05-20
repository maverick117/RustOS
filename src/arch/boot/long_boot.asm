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
  int 0x01
  call init_text_console          ; Setup text console

  mov qword rcx, 0x0
  mov qword rbx, assembly_success_msg_len - assembly_success_msg
.loop:
  xor rax, rax
  mov byte al, [assembly_success_msg + rcx]
  mov byte ah, 0x2f
  mov qword rdi, rax
  push rcx
  call print_char
  pop rcx
  inc rcx
  cmp rcx, rbx
  jl .loop

.call_rust:
  xor rax, rax
  xor rbx, rbx
  xor rcx, rcx
  xor rdx, rdx
  jmp rust_start                 ; Start Rust Kernel Section

  hlt ; Prevent returning into another section.

  section .data
assembly_success_msg:
  db 0x0a, "BOOT GOOD", 0x0a, 0x0a,"Assembly boot successful. Handing control to Rust code...", 0x0a
assembly_success_msg_len:
