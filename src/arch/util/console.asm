global init_text_console
global print_char
global clear_console

section .text
bits 64

; Init text console, test clearing screen and printing characters

init_text_console:
    push rcx
    push rbx
    push rax

    call clear_console
    ; UNIX SYSV ABI calling convention

    mov qword rcx, 0x0
    mov qword rbx, console_test_msg_len - console_test_msg

.loop:
    xor rax, rax
    mov byte al, [console_test_msg + rcx]
    mov byte ah, 0x0f
    mov qword rdi, rax
    call print_char
    inc rcx
    cmp rcx, rbx
    jl .loop

    pop rax
    pop rbx
    pop rcx
    ret




; Internal function to clear screen, completed

clear_console:
    push rcx
    push rax
    
    mov qword rcx, 0x0 ; Init 
    mov qword rax, [console_width] ; Move value of console_width to rax
    mul qword [console_height] ; Multiply it with console_height
    jmp clear_console.compare_counter
.clear_loop:
    mov dword [0xb8000 + 2*rcx], 0x00000000 ; Clear console
    inc rcx
.compare_counter:
    cmp rcx, rax
    jl .clear_loop

    pop rax
    pop rcx
    ret


; Internal function to scroll down one line

scroll_one_line: ; Scroll down one line
    push rcx ; Save registers used within this function 
    push rax
    push rbx

    mov qword rcx, 0x0;
    mov qword rax, [console_height]
    dec rax
    mul qword [console_width]
    jmp scroll_one_line.compare_counter
.scroll_loop:
    mov dword ebx, [0xb80A0 + 2*rcx]
    mov dword [0xb8000 + 2*rcx], ebx
    inc rcx
    ;inc rcx
.compare_counter:
    cmp rcx, rax
    jl .scroll_loop
    mov dword [current_position], 0xb8000 + 24 * 80 * 2

    mov qword rcx, 0x0;
.clear_last_line:
   
    mov word [0xb8f00 + rcx * 2] , 0x00
    inc rcx
    

    cmp rcx, 0xA0
    jl .clear_last_line

    pop rbx ; Restore registers before call
    pop rax
    pop rcx
    ret

; External function to print out a character with a color

print_char: ; void print_char(short word);
    push rax
    push rbx

    mov dword eax, [current_position] ; Get the address to print to
    mov qword rbx, rdi ; Move the first argument
    cmp bl, 0x0a
    je .print_new_line
    mov word [eax], bx
    add eax, 2 ; Add 2 to the address
    cmp eax, 0xb8000 + 25 * 80 * 2
    je .print_new_line
    mov dword [current_position], eax
.end:
    pop rbx
    pop rax
    ret
.print_new_line:
    call scroll_one_line
    jmp .end


section .data

console_width:
    dq 80
console_height:
    dq 25
current_position:
    dq 0xb8000 + 24 * 80 * 2

console_test_msg:
    db "Text console test...", 0x0a, "If you see two lines of text, then the test is successful",0x0a
console_test_msg_len: