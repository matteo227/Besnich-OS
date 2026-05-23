BITS 32

SECTION .data
disk_buffer resb 512

SECTION .text
GLOBAL storage_read

storage_read:

    mov eax, 70
    mov ebx, disk_buffer
    int 0x80

    ret
