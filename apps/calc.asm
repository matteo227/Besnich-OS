BITS 32

SECTION .data

title db 'CALCOLATRICE BESNICH OS',0
msg1  db 'A (0-9): ',0
msg2  db 'B (0-9): ',0
res   db 'Risultato: ',0

newline db 10,0

SECTION .text
GLOBAL calc_app

calc_app:

    call print_newline
    mov esi, title
    call print_string

    call print_newline
    mov esi, msg1
    call print_string
    call read_char
    sub al, '0'
    mov bl, al

    call print_newline
    mov esi, msg2
    call print_string
    call read_char
    sub al, '0'

    add al, bl

    call print_newline
    mov esi, res
    call print_string

    add al, '0'
    mov ah, 0x0E
    int 0x10

    ret
