BITS 32

SECTION .bss
buffer resb 256

SECTION .data

title db 'BESNICH NOTEPAD',0
msg   db 'Scrivi testo (ENTER per fine):',0
saved db 'Salvato in memoria',0

newline db 10,0

SECTION .text
GLOBAL notepad_app

notepad_app:

    call clear_screen

    mov esi, title
    call print_string
    call print_newline

    mov esi, msg
    call print_string
    call print_newline

    mov edi, buffer

read_loop:
    call read_char
    cmp al, 13
    je done

    mov [edi], al
    inc edi
    jmp read_loop

done:
    mov byte [edi], 0

    call print_newline
    mov esi, saved
    call print_string

    ret
