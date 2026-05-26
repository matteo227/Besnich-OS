BITS 32

SECTION .data

title db '[NETWORK ERROR]',0
msg1 db 'Connessione non disponibile.',0
code db 'ERROR: NETWORK_DISCONNECTED',0

SECTION .text
GLOBAL network_error
