BITS 32

SECTION .text
GLOBAL emoji_check

emoji_check:

    cmp al, ':'
    je emoji_detect

    ret


emoji_detect:

    mov eax, 250
    int 0x80

    ret
