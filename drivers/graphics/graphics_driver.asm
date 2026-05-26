BITS 32

SECTION .text
GLOBAL graphics_init

graphics_init:

    call set_vga_mode
    ret
