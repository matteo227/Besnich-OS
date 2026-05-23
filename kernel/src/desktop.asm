[BITS 32]
[ORG 0x100000]

desktop_start:
    ; Passa a modalità VESA 1024x768 32bit
    mov ax, 0x4F02
    mov bx, 0x4117
    int 0x10

    ; Sfondo blu scuro Besnich
    call draw_background

    ; Taskbar in basso
    call draw_taskbar

    ; Icone desktop
    call draw_icons

    ; Ora/data
    call draw_datetime

    ; Loop eventi
desktop_loop:
    call check_mouse
    call check_keyboard
    jmp desktop_loop

draw_background:
    mov esi, 0xFD000000
    mov edi, 0xFD000000
    mov ecx, 768000
    mov eax, 0x00101C28
    rep stosd
    ret

draw_taskbar:
    ; Barra in basso nera semi-trasparente
    mov esi, 0xFD000000
    mov edi, 768000
    sub edi, 30720
    mov ecx, 30720
    mov eax, 0xCC000000
    rep stosd

    ; Pulsante Start Besnich
    mov dh, 43
    mov dl, 0
    mov si, start_text
    call draw_button_green

    ; Icone taskbar: Wi-Fi, volume, batteria, ora
    mov dh, 43
    mov dl, 70
    mov si, wifi_icon
    call draw_taskbar_icon

    mov dh, 43
    mov dl, 80
    mov si, vol_icon
    call draw_taskbar_icon

    mov dh, 43
    mov dl, 90
    mov si, batt_icon
    call draw_taskbar_icon

    ret

draw_icons:
    ; Icona Terminale
    mov dh, 2
    mov dl, 2
    mov si, terminal_icon
    call draw_icon
    mov dh, 10
    mov dl, 2
    mov si, term_label
    call draw_label

    ; Icona File Manager
    mov dh, 2
    mov dl, 8
    mov si, folder_icon
    call draw_icon
    mov dh, 10
    mov dl, 8
    mov si, folder_label
    call draw_label

    ; Icona Impostazioni
    mov dh, 2
    mov dl, 14
    mov si, settings_icon
    call draw_icon
    mov dh, 10
    mov dl, 14
    mov si, settings_label
    call draw_label

    ; Icona Browser
    mov dh, 2
    mov dl, 20
    mov si, browser_icon
    call draw_icon
    mov dh, 10
    mov dl, 18
    mov si, browser_label
    call draw_label

    ; Icona Scudo Sicurezza
    mov dh, 2
    mov dl, 26
    mov si, shield_icon
    call draw_icon
    mov dh, 10
    mov dl, 25
    mov si, shield_label
    call draw_label

    ; Icona Cestino
    mov dh, 2
    mov dl, 120
    mov si, trash_icon
    call draw_icon
    mov dh, 10
    mov dl, 119
    mov si, trash_label
    call draw_label

    ret

draw_datetime:
    ; Leggi RTC
    mov al, 0x04
    out 0x70, al
    in al, 0x71
    mov [hours], al

    mov al, 0x02
    out 0x70, al
    in al, 0x71
    mov [minutes], al

    ; Mostra ora in alto a destra
    mov dh, 1
    mov dl, 120
    mov si, time_str
    call draw_label_right

    ; Mostra data
    mov dh, 1
    mov dl, 115
    mov si, date_str
    call draw_label_right

    ret

draw_button_green:
    push edx
    ; Sfondo verde bottone
    mov ecx, 40
    mov eax, 0x00AA5500
    rep stosd
    ; Testo
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0F
    int 0x10
    pop edx
    ret

draw_taskbar_icon:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0B
    int 0x10
    ret

draw_icon:
    push edx
    ; Quadrato icona 48x48
    mov eax, 0x00FFD700
    mov ecx, 2304
    rep stosd
    pop edx
    ret

draw_label:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0F
    int 0x10
    ret

draw_label_right:
    mov ah, 0x13
    mov al, 0x01
    mov bh, 0
    mov bl, 0x0B
    int 0x10
    ret

check_mouse:
    mov ax, 0x0003
    int 0x33
    ret

check_keyboard:
    mov ah, 0x01
    int 0x16
    jz no_key
    mov ah, 0x00
    int 0x16
    cmp al, 0x1B
    je show_shutdown_menu
no_key:
    ret

show_shutdown_menu:
    ; Overlay spegnimento
    mov eax, 0xCC000000
    mov ecx, 768000
    rep stosd

    mov dh, 15
    mov dl, 45
    mov si, shutdown_header
    call draw_label

    mov dh, 18
    mov dl, 45
    mov si, shutdown_opt1
    call draw_label

    mov dh, 20
    mov dl, 45
    mov si, shutdown_opt2
    call draw_label

    mov dh, 22
    mov dl, 45
    mov si, shutdown_opt3
    call draw_label

    ret

start_text     db "Avvia", 0
wifi_icon      db chr(0xDB), " ", 0
vol_icon       db chr(0xDB), " ", 0
batt_icon      db chr(0xDB), " ", 0

terminal_icon  db ">_", 0
term_label     db "Terminale", 0
folder_icon    db "[]", 0
folder_label   db "File", 0
settings_icon  db chr(0x13), " ", 0
settings_label db "Impostazioni", 0
browser_icon   db "O", 0
browser_label  db "Browser Sicuro", 0
shield_icon    db chr(0x0F), " ", 0
shield_label   db "Besnich Protect", 0
trash_icon     db "X", 0
trash_label    db "Cestino", 0

shutdown_header db "BESNICH OS", 0
shutdown_opt1  db "[1] Spegni", 0
shutdown_opt2  db "[2] Riavvia", 0
shutdown_opt3  db "[3] Annulla", 0

hours    db 0
minutes  db 0
seconds  db 0
time_str db "00:00", 0
date_str db "01/01/2026", 0
