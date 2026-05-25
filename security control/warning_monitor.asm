BITS 32

SECTION .data

title db 'BESNICH PROTECT WARNING',0
msg1  db 'Attivita sospetta rilevata',0
msg2  db 'Premi X per chiudere',0

border1 db '========================================',0

newline db 10,0

SECTION .text
GLOBAL warning_monitor

warning_monitor:

main_loop:

    mov eax, 160
    mov ebx, activity_buffer
    int 0x80

    mov esi, activity_buffer
    call scan_activity

    jmp main_loop


scan_activity:

.loop:

    mov al, [esi]

    cmp al, 0
    je done

    cmp al, 'x'
    je warning_popup

    inc esi
    jmp .loop


warning_popup:

    call clear_screen

    mov esi, border1
    call print_string
    call print_newline

    mov esi, title
    call print_string
    call print_newline

    mov esi, border1
    call print_string
    call print_newline

    mov esi, msg1
    call print_string
    call print_newline

    mov esi, msg2
    call print_string
    call print_newline

wait_close:

    call read_char

    cmp al, 'x'
    je close_popup

    cmp al, 'X'
    je close_popup

    jmp wait_close


close_popup:

    call clear_screen
    ret


done:
    ret


SECTION .bss
activity_buffer resb 4096


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


read_char:

    mov ah, 0x00
    int 0x16

    ret


clear_screen:

    mov ax, 0x0003
    int 0x10

    ret
