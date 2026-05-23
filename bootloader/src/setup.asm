[BITS 16]
[ORG 0x7E00]

setup_start:
    call language_selection
    call wifi_selection
    call main_menu
    jmp $

language_selection:
    mov ax, 0x0600
    mov bh, 0x01
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 3
    mov dl, 25
    mov si, lang_header
    call print_yellow

    mov dh, 7
    mov dl, 20
    mov si, lang_opt1
    call print_white

    mov dh, 9
    mov dl, 20
    mov si, lang_opt2
    call print_white

    mov dh, 11
    mov dl, 20
    mov si, lang_opt3
    call print_white

    mov dh, 14
    mov dl, 20
    mov si, lang_prompt
    call print_cyan

selected_lang:
    mov ah, 0x00
    int 0x16

    cmp al, '1'
    je set_italian
    cmp al, '2'
    je set_english
    cmp al, '3'
    je set_french
    jmp selected_lang

set_italian:
    mov byte [language], 0
    mov si, lang_it
    call store_language
    ret

set_english:
    mov byte [language], 1
    mov si, lang_en
    call store_language
    ret

set_french:
    mov byte [language], 2
    mov si, lang_fr
    call store_language
    ret

store_language:
    mov di, lang_selected
store_lang_loop:
    lodsb
    cmp al, 0
    je store_lang_done
    stosb
    jmp store_lang_loop
store_lang_done:
    mov al, 0
    stosb
    ret

wifi_selection:
    mov ax, 0x0600
    mov bh, 0x01
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 3
    mov dl, 20
    mov si, wifi_header
    call print_yellow

    mov dh, 6
    mov dl, 20
    mov si, scanning_msg
    call print_white

    call delay_medium

    mov ax, 0x0600
    mov bh, 0x01
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 3
    mov dl, 20
    mov si, wifi_header
    call print_yellow

    mov dh, 6
    mov dl, 20
    mov si, net1
    call print_cyan

    mov dh, 8
    mov dl, 20
    mov si, net2
    call print_cyan

    mov dh, 10
    mov dl, 20
    mov si, net3
    call print_cyan

    mov dh, 12
    mov dl, 20
    mov si, net4
    call print_cyan

    mov dh, 15
    mov dl, 20
    mov si, wifi_prompt
    call print_white

selected_net:
    mov ah, 0x00
    int 0x16

    cmp al, '1'
    je connect_net1
    cmp al, '2'
    je connect_net2
    cmp al, '3'
    je connect_net3
    cmp al, '4'
    je connect_net4
    jmp selected_net

connect_net1:
    mov si, net1_name
    call store_ssid
    jmp ask_password

connect_net2:
    mov si, net2_name
    call store_ssid
    jmp ask_password

connect_net3:
    mov si, net3_name
    call store_ssid
    jmp ask_password

connect_net4:
    mov si, net4_name
    call store_ssid
    jmp ask_password

store_ssid:
    mov di, wifi_ssid
store_ssid_loop:
    lodsb
    cmp al, 0
    je store_ssid_done
    stosb
    jmp store_ssid_loop
store_ssid_done:
    mov al, 0
    stosb
    ret

ask_password:
    mov ax, 0x0600
    mov bh, 0x01
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 5
    mov dl, 15
    mov si, pass_msg
    call print_white

    mov dh, 9
    mov dl, 15
    mov si, pass_input_msg
    call print_white

    mov di, wifi_pass
    mov byte [wifi_pass], 0

read_pass:
    mov ah, 0x00
    int 0x16

    cmp al, 0x0D
    je pass_done

    cmp al, 0x08
    je pass_backspace

    stosb
    mov ah, 0x0E
    mov al, '*'
    int 0x10
    jmp read_pass

pass_backspace:
    cmp di, wifi_pass
    je read_pass
    dec di
    mov al, ' '
    mov ah, 0x0E
    int 0x10
    mov al, 0x08
    int 0x10
    jmp read_pass

pass_done:
    mov al, 0
    stosb

    mov ax, 0x0600
    mov bh, 0x02
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 6
    mov dl, 15
    mov si, connecting_msg
    call print_white

    call delay_medium

    mov dh, 6
    mov dl, 50
    mov si, connected_msg
    call print_green

    call delay_short
    ret

main_menu:
    mov ax, 0x0600
    mov bh, 0x01
    mov cx, 0x0000
    mov dx, 0x184F
    int 0x10

    mov dh, 2
    mov dl, 25
    mov si, menu_header
    call print_yellow

    mov dh, 5
    mov dl, 20
    mov si, menu_sub
    call print_white

    mov dh, 9
    mov dl, 20
    mov si, opt1_msg
    call print_cyan

    mov dh, 11
    mov dl, 20
    mov si, opt2_msg
    call print_cyan

    mov dh, 13
    mov dl, 20
    mov si, opt3_msg
    call print_cyan

    mov dh, 16
    mov dl, 20
    mov si, input_msg
    call print_white

    mov ah, 0x00
    int 0x16

    cmp al, '1'
    je fresh_install
    cmp al, '2'
    je verify_system
    cmp al, '3'
    je recovery_mode
    jmp main_menu

print_yellow:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0E
    int 0x10
    ret

print_white:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0F
    int 0x10
    ret

print_cyan:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0B
    int 0x10
    ret

print_green:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0A
    int 0x10
    ret

delay_short:
    mov cx, 0x02
    mov dx, 0x0000
    mov ah, 0x86
    int 0x15
    ret

delay_medium:
    mov cx, 0x05
    mov dx, 0x0000
    mov ah, 0x86
    int 0x15
    ret

lang_header   db "BESNICH OS - SELEZIONE LINGUA", 0
lang_opt1     db "[1] Italiano", 0
lang_opt2     db "[2] English", 0
lang_opt3     db "[3] Francais", 0
lang_prompt   db "Seleziona lingua [1-3]: ", 0
lang_it       db "Italiano", 0
lang_en       db "English", 0
lang_fr       db "Francais", 0

wifi_header   db "BESNICH OS - CONNESSIONE WI-FI", 0
scanning_msg  db "Scansione reti disponibili...", 0
net1          db "[1] BesnichSecure_5G", 0
net2          db "[2] HomeNetwork", 0
net3          db "[3] Guest_WiFi", 0
net4          db "[4] Saltala (configura dopo)", 0
wifi_prompt   db "Seleziona rete [1-4]: ", 0
net1_name     db "BesnichSecure_5G", 0
net2_name     db "HomeNetwork", 0
net3_name     db "Guest_WiFi", 0
net4_name     db "", 0
pass_msg      db "Inserisci password Wi-Fi:", 0
pass_input_msg db "Password: ", 0
connecting_msg db "Connessione in corso...", 0
connected_msg db "[CONNESSO]", 0

menu_header   db "BESNICH OS - SETUP", 0
menu_sub      db "Lingua e Wi-Fi configurati. Scegli:", 0
opt1_msg      db "[1] Installa Besnich OS (fresh install)", 0
opt2_msg      db "[2] Verifica integrita sistema", 0
opt3_msg      db "[3] Recovery mode", 0
input_msg     db "Seleziona [1-3]: ", 0

language      db 0
lang_selected times 20 db 0
wifi_ssid     times 32 db 0
wifi_pass     times 64 db 0

fresh_install:
    ret

verify_system:
    ret

recovery_mode:
    ret

times 8192-($-$$) db 0

    
