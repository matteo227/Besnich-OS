BITS 32

SECTION .data

safe_msg db '[APP SAFE]',0
warn_msg db '[SUSPICIOUS APP DETECTED]',0

sig1 db 'inject',0
sig2 db 'autorun',0
sig3 db 'hidden',0
sig4 db 'keylog',0

newline db 10,0

SECTION .text
GLOBAL heuristic_scan

heuristic_scan:

    mov esi, ebx

scan_loop:

    mov al, [esi]

    cmp al, 0
    je safe

    cmp al, 'i'
    je suspicious

    cmp al, 'a'
    je suspicious

    cmp al, 'k'
    je suspicious

    inc esi
    jmp scan_loop


suspicious:

    mov esi, warn_msg
    call print_string
    call print_newline

    call popup_warning

    call cloud_scan

    ret


safe:

    mov esi, safe_msg
    call print_string
    call print_newline

    ret
