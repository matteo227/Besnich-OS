BITS 32

SECTION .text
GLOBAL encrypt_shadow

encrypt_shadow:

    mov eax, 271
    int 0x80

    ret
