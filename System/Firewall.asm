BITS 32

SECTION .data

title db 'BESNICH SMART FIREWALL ACTIVE',0

allow_msg db '[CONNECTION ALLOWED]',0
block_msg db '[SUSPICIOUS CONNECTION BLOCKED]',0

scan_msg db '[ANALYZING NETWORK TRAFFIC]',0

sig1 db 'inject',0
sig2 db 'malware',0
sig3 db 'trojan',0
sig4 db 'botnet',0

newline db 10,0

SECTION .text
GLOBAL firewall_main

firewall_main:

    mov esi, title
    call print_string
    call print_newline

main_loop:

    mov eax, 220
    mov ebx, packet_buffer
    int 0x80

    mov esi, scan_msg
    call print_string
    call print_newline

    mov esi, packet_buffer
    call inspect_packet

    jmp main_loop


inspect_packet:

.loop:

    mov al, [esi]

    cmp al, 0
    je allow

    cmp al, 'm'
    je block

    cmp al, 't'
    je block

    inc esi
    jmp .loop


allow:

    mov esi, allow_msg
    call print_string
    call print_newline

    mov eax, 221
    int 0x80

    ret


block:

    mov esi, block_msg
    call print_string
    call print_newline

    call popup_warning

    mov eax, 222
    int 0x80

    ret
