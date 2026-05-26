BITS 32

SECTION .data

title db 'BESNICH OS NETWORK FAILURE',0
msg1 db 'Stack rete danneggiato.',0
msg2 db 'Connessione terminata.',0
code db 'ERROR CODE: NETWORK_FATAL',0

SECTION .text
GLOBAL network_failure
