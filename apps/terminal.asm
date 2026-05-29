BITS 32

SECTION .text
GLOBAL terminal_main

terminal_main:

    call read_command

    call parse_command

    ret
