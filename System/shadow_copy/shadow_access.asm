BITS 32

SECTION .data

deny_msg db '[SHADOW ACCESS DENIED]',0

SECTION .text
GLOBAL verify_shadow_access

verify_shadow_access:

    mov eax, 272
    int 0x80

    cmp eax, 1
    je allow

deny:

    mov esi, deny_msg
    call print_string

    ret

allow:
    ret
