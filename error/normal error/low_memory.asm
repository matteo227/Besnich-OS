BITS 32

SECTION .data

title db '[MEMORY WARNING]',0
msg1 db 'Memoria insufficiente.',0
code db 'ERROR: LOW_MEMORY',0

SECTION .text
GLOBAL low_memory
