BITS 32

SECTION .data

title db 'BESNICH PROTECT REAL-TIME ACTIVE',0
safe  db '[SAFE]',0
block db '[BLOCKED THREAT]',0
scan  db 'Scanning...',0

sig1 db '<script>',0
sig2 db 'virus',0

newline db 10,0

SECTION .text
GLOBAL protect_realtime

protect_realtime:

    call clear_screen

    mov esi, title
    call print_string
    call print_newline

main_loop:

    ; prende input/buffer globale (browser, memoria, ecc.)
    mov esi, [watch_buffer]

    call scan_memory

    jmp main_loop


scan_memory:

.loop:
    mov al, [esi]
    cmp al, 0
    je safe_state

    cmp al, '<'
    je threat_detected

    inc esi
    jmp .loop


threat_detected:

    mov esi, block
    call print_string
    call print_newline

    mov eax, 99
    int 0x80

    jmp main_loop


safe_state:

    mov esi, safe
    call print_string
    call print_newline

    mov eax, 100
    int 0x80

    jmp main_loop


SECTION .bss
watch_buffer resd 1


print_string:
.next:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp .next
.done:
    ret


print_newline:
    mov ah, 0x0E
    mov al, 10
    int 0x10
    ret


clear_screen:
    mov ax, 0x0003
    int 0x10
    ret
