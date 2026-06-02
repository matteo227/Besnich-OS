BITS 32

SECTION .text
GLOBAL video_risk_engine

video_risk_engine:

    mov eax, 305
    int 0x80

    ret
