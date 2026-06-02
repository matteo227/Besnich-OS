BITS 32

SECTION .text
GLOBAL action_manager

action_manager:

    mov eax, 306
    int 0x80

    ret
