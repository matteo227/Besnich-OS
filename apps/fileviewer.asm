BITS 32

SECTION .data

title db 'BESNICH FILE VIEWER',0
msg   db 'Contenuto file:',0
newline db 10,0

SECTION .bss
filebuf resb 256

SECTION .text
GLOBAL fileviewer_app

fileviewer_app:

    call clear_screen

    mov esi, title
    call print_string
    call print_newline

    ; syscall load file
    mov eax, 80
    mov ebx, filebuf
    int 0x80

    mov esi, msg
    call print_string
    call print_newline

    mov esi, filebuf
    call print_string

    ret
