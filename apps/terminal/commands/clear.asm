BITS 32

SECTION .text
GLOBAL clear_command

clear_command:

    mov ax, 0x0003
    int 0x10

    ret
