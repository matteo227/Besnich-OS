BITS 32

SECTION .data
key_buffer db 0

SECTION .text
GLOBAL keyboard_driver

keyboard_driver:

    mov ah, 0x00
    int 0x16

    mov [key_buffer], al

    mov eax, 10
    mov ebx, key_buffer
    int 0x80

    ret
