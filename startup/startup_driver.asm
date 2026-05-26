BITS 32

SECTION .text
GLOBAL startup_init

startup_init:

    call video_init
    call storage_init

    ret
