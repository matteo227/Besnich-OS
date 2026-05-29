BITS 32

SECTION .text
GLOBAL shutdown_command

shutdown_command:

    cli
    hlt
