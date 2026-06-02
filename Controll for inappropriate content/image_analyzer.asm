BITS 32

SECTION .text
GLOBAL image_analyzer

image_analyzer:

    mov eax, 303
    int 0x80

    ret
