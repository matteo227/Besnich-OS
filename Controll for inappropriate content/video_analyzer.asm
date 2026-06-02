BITS 32

SECTION .text
GLOBAL video_analyzer

video_analyzer:

    call video_frame_scan
    call video_risk_engine

    ret
