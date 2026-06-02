BITS 32

SECTION .text
GLOBAL ai_service

ai_service:

main_loop:

    call file_scanner
    call text_analyzer
    call image_analyzer
    call video_analyzer
    call risk_engine
    call action_manager

    mov eax, 300
    int 0x80

    jmp main_loop
