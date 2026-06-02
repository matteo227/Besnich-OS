BITS 32

SECTION .text
GLOBAL file_scanner

file_scanner:

    mov eax, 301
    int 0x80

    ret
