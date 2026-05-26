BITS 32

SECTION .text
GLOBAL integrity_check

integrity_check:

    mov eax, 231
    int 0x80

    ret
