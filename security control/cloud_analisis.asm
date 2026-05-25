BITS 32

SECTION .data

vt_msg db 'VirusTotal scan...',0
ha_msg db 'Hybrid Analysis scan...',0

clean_msg db '[CLEAN]',0
bad_msg db '[MALWARE DETECTED]',0

newline db 10,0

SECTION .text
GLOBAL cloud_scan

cloud_scan:

    mov esi, vt_msg
    call print_string
    call print_newline

    mov eax, 200
    mov ebx, app_buffer
    int 0x80

    mov esi, ha_msg
    call print_string
    call print_newline

    mov eax, 201
    mov ebx, app_buffer
    int 0x80

    cmp eax, 1
    je malware

clean:

    mov esi, clean_msg
    call print_string
    call print_newline

    ret


malware:

    mov esi, bad_msg
    call print_string
    call print_newline

    mov eax, 202
    int 0x80

    ret


SECTION .bss
app_buffer resb 8192
