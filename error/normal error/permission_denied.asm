BITS 32

SECTION .data

title db '[PERMISSION ERROR]',0
msg1 db 'Permesso negato.',0
code db 'ERROR: ACCESS_DENIED',0

SECTION .text
GLOBAL permission_denied
