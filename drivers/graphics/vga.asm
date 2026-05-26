BITS 32

SECTION .text
GLOBAL set_vga_mode

set_vga_mode:

    mov ax, 0x0013
    int 0x10

    ret
