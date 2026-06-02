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
    call video_frame_scan.asm
    call video_risk_engine.asm

    mov eax, 300
    int 0x80

    jmp main_loop
