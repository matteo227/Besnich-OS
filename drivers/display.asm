BITS 32

SECTION .text
GLOBAL display_driver

display_driver:

    mov ah, 0x0E
    mov al, bl
    int 0x10

    ret
