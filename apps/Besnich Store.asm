BITS 32

SECTION .data

top    db '╔════════════════════════════════════════╗',0
title  db '║        BESNICH APP STORE v1.0          ║',0
mid    db '╠════════════════════════════════════════╣',0
line1  db '║  [1] Installa / Aggiorna App           ║',0
line2  db '║  [2] Update App da Sito Ufficiale      ║',0
line3  db '║  [3] Esci                              ║',0
bottom db '╚════════════════════════════════════════╝',0

prompt db '> Seleziona opzione: ',0

newline db 10,0

SECTION .text
GLOBAL store_ui

store_ui:

    call clear_screen

    mov esi, top
    call print_string
    call print_newline

    mov esi, title
    call print_string
    call print_newline

    mov esi, mid
    call print_string
    call print_newline

    mov esi, line1
    call print_string
    call print_newline

    mov esi, line2
    call print_string
    call print_newline

    mov esi, line3
    call print_string
    call print_newline

    mov esi, bottom
    call print_string
    call print_newline

    mov esi, prompt
    call print_string

    call read_char
    ret


print_string:
.next:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp .next
.done:
    ret


print_newline:
    mov ah, 0x0E
    mov al, 10
    int 0x10
    ret


clear_screen:
    mov ax, 0x0003
    int 0x10
    ret
