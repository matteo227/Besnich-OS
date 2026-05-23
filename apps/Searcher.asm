BITS 32

SECTION .data

topbar  db '========================================',0
title   db '   SEARCHER - Secure Web OS',0
bar     db '========================================',0

urlbox  db '[ URL ] > ',0
pagebox db '[ PAGE LOADED AREA ]',0

newline db 10,0

SECTION .text
GLOBAL browser_ui

browser_ui:

    call clear_screen

    mov esi, topbar
    call print_string
    call print_newline

    mov esi, title
    call print_string
    call print_newline

    mov esi, bar
    call print_string
    call print_newline

    mov esi, urlbox
    call print_string

    call read_url

    call print_newline

    mov esi, bar
    call print_string
    call print_newline

    mov esi, pagebox
    call print_string

    ret


read_url:

    mov edi, url_buffer

.loop:
    call read_char
    cmp al, 13
    je .done

    mov [edi], al
    inc edi
    jmp .loop

.done:
    mov byte [edi], 0
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


SECTION .bss
url_buffer resb 64
