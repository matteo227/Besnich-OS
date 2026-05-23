BITS 32

SECTION .data

menu_title db '=== BESNICH OS SETTINGS ===',0

menu_1 db '1. Sicurezza',0
menu_2 db '2. Aggiornamenti',0
menu_3 db '3. WiFi',0
menu_4 db '4. Bluetooth',0
menu_5 db '5. Esci',0

prompt db 'Scelta: ',0

msg_secure db 'Protezione sistema attiva',0

checking_updates db 'Controllo aggiornamenti...',0
github_msg       db 'Connessione GitHub...',0
update_found     db 'Aggiornamento trovato!',0
download_msg     db 'Download update...',0

msg_wifi db 'WiFi attivato',0
msg_bt   db 'Bluetooth attivato',0
msg_exit db 'Chiusura impostazioni...',0

github_url db 'https://raw.githubusercontent.com/matteo227/Besnich-OS/main/update.bin',0

newline db 10,0

SECTION .text
GLOBAL _start

_start:
    call clear_screen

main_menu:

    call print_newline
    mov esi, menu_title
    call print_string

    call print_newline
    mov esi, menu_1
    call print_string

    call print_newline
    mov esi, menu_2
    call print_string

    call print_newline
    mov esi, menu_3
    call print_string

    call print_newline
    mov esi, menu_4
    call print_string

    call print_newline
    mov esi, menu_5
    call print_string

    call print_newline
    mov esi, prompt
    call print_string

    call read_char

    cmp al, '1'
    je security_menu

    cmp al, '2'
    je updates_menu

    cmp al, '3'
    je wifi_menu

    cmp al, '4'
    je bluetooth_menu

    cmp al, '5'
    je exit_app

    jmp main_menu

security_menu:
    call print_newline
    mov esi, msg_secure
    call print_string
    jmp main_menu

updates_menu:

    call print_newline
    mov esi, checking_updates
    call print_string

    call github_check

    call print_newline
    mov esi, update_found
    call print_string

    call download_update

    jmp main_menu

wifi_menu:
    call print_newline
    mov esi, msg_wifi
    call print_string
    jmp main_menu

bluetooth_menu:
    call print_newline
    mov esi, msg_bt
    call print_string
    jmp main_menu

exit_app:
    call print_newline
    mov esi, msg_exit
    call print_string
    ret

github_check:

    call print_newline
    mov esi, github_msg
    call print_string

    mov eax, 5
    mov ebx, github_url
    int 0x80

    ret

download_update:

    call print_newline
    mov esi, download_msg
    call print_string

    mov eax, 6
    mov ebx, github_url
    int 0x80

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
    mov esi, newline
    call print_string
    ret

read_char:
    mov ah, 0x00
    int 0x16
    ret

clear_screen:
    mov ax, 0x0003
    int 0x10
    ret
