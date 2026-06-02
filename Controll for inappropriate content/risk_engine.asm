BITS 32

SECTION .text
GLOBAL risk_engine

risk_engine:

    mov eax, 305
    int 0x80

    ret
