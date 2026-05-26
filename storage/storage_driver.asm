BITS 32

SECTION .bss
disk_buffer resb 512

SECTION .text
GLOBAL storage_init
GLOBAL disk_read
GLOBAL disk_write

storage_init:

    ret


disk_read:

    ; eax = sector
    ; ebx = buffer

    mov ecx, eax

    mov eax, 240
    int 0x80

    ret


disk_write:

    ; eax = sector
    ; ebx = buffer

    mov ecx, eax

    mov eax, 241
    int 0x80

    ret
