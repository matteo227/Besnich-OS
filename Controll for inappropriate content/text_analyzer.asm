BITS 32

SECTION .text
GLOBAL text_analyzer

text_analyzer:

    mov eax, 302
    int 0x80

    ret
