BITS 32

SECTION .text
GLOBAL video_init
GLOBAL put_pixel

VIDEO_MEMORY equ 0xA0000

video_init:

    mov ax, 0x0013
    int 0x10

    ret


put_pixel:

    ; eax = x
    ; ebx = y
    ; cl  = color

    mov edi, VIDEO_MEMORY

    mov edx, 320
    mul ebx
    add eax, ebx

    add edi, eax

    mov [edi], cl

    ret
