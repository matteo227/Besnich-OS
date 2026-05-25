BITS 32

SECTION .data

title db 'BESNICH SCREEN MONITOR ACTIVE',0
warn  db '[ADWARE DETECTED]',0
safe  db '[SCREEN SAFE]',0

popup_sig db 'popup',0
ad_sig    db 'ads',0

newline db 10,0

SECTION .bss
screen_buffer resb 65536

SECTION .text
GLOBAL screen_monitor

screen_monitor:

main_loop:

    mov eax, 140
    mov ebx, screen_buffer
    int 0x80

    mov esi, screen_buffer
    call scan_screen

    jmp main_loop


scan_screen:

.loop:
    mov al, [esi]

    cmp al, 0
    je clean

    cmp al, 'a'
    je suspicious

    inc esi
    jmp .loop


suspicious:

    mov esi, warn
    call print_string
    call print_newline

    mov eax, 141
    mov ebx, screen_buffer
    int 0x80

    ret


clean:

    mov esi, safe
    call print_string
    call print_newline

    ret


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
