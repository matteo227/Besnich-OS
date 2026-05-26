BITS 32

VIDEO_MEMORY equ 0xA0000

SECTION .text
GLOBAL put_pixel

put_pixel:

    mov edi, VIDEO_MEMORY

    mov edx, 320
    mul ebx
    add eax, ebx

    add edi, eax

    mov [edi], cl

    ret
