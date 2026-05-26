BITS 32

SECTION .data

title db '[APPLICATION ERROR]',0
msg1 db 'L applicazione si e chiusa in modo anomalo.',0
code db 'ERROR: APP_CRASH',0

SECTION .text
GLOBAL app_crash
