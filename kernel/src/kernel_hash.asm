BITS 32

SECTION .text
GLOBAL kernel_hash

kernel_hash:

    mov eax, 233
    int 0x80

    ret
