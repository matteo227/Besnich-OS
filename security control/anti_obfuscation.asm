BITS 32

SECTION .data

title db '[ANTI OBFUSCATION ACTIVE]',0
safe  db '[CODE CLEAN]',0
warn  db '[OBFUSCATED CODE DETECTED]',0

sig1 db 'xor',0
sig2 db 'base64',0
sig3 db 'packed',0
sig4 db 'shell',0

newline db 10,0

SECTION .text
GLOBAL anti_obfuscation_scan

anti_obfuscation_scan:

    mov esi, title
    call print_string
    call print_newline

    mov esi, ebx

scan_loop:

    mov al, [esi]

    cmp al, 0
    je clean

    cmp al, 'x'
    je detected

    cmp al, 'b'
    je detected

    cmp al, 'p'
    je detected

    inc esi
    jmp scan_loop


detected:

    mov esi, warn
    call print_string
    call print_newline

    mov eax, 210
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
