BITS 32

SECTION .data

title db '[BESNICH PATCH GUARD ACTIVE]',0

safe_msg db '[KERNEL SAFE]',0
warn_msg db '[UNAUTHORIZED PATCH DETECTED]',0

newline db 10,0

SECTION .text
GLOBAL patch_guard

patch_guard:

main_loop:

    call integrity_check

    cmp eax, 1
    je compromised

safe:

    mov esi, safe_msg
    call print_string
    call print_newline

    jmp continue_loop


compromised:

    mov esi, warn_msg
    call print_string
    call print_newline

    mov eax, 230
    int 0x80

continue_loop:

    jmp main_loop
