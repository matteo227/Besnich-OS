BITS 32

SECTION .text
GLOBAL video_frame_scan

video_frame_scan:

    mov eax, 304
    int 0x80

    ret
