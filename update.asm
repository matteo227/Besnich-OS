BITS 32

SECTION .data

update_data db 'BESNICH_UPDATE',0
version     db '1.1',0
url         db 'https://raw.githubusercontent.com/matteo277/Besnich-OS/main/kernel.bin',0

SECTION .text
GLOBAL _start

_start:
    nop
