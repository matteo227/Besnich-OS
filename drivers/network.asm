BITS 32

SECTION .text
GLOBAL network_driver

network_driver:

    mov eax, 90
    mov ebx, esi
    mov ecx, edi
    int 0x80

    ret
