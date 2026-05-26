BITS 32

SECTION .text
GLOBAL driver_verify

driver_verify:

    mov eax, 232
    int 0x80

    ret
