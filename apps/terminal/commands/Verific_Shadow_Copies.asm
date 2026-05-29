BITS 32

SECTION .data

msg1 db '[VERIFYING SHADOW COPIES]',0
msg2 db '[SCAN COMPLETE]',0

newline db 10,0

SECTION .text
GLOBAL verific_shadow_copies

verific_shadow_copies:

    mov esi, msg1
    call print_string
    call print_newline

    mov eax, 280
    int 0x80

    mov esi, msg2
    call print_string
    call print_newline

    ret
