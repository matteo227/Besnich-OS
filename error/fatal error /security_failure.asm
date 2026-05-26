BITS 32

SECTION .data

title db 'BESNICH SECURITY FAILURE',0
msg1 db 'Minaccia critica rilevata.',0
msg2 db 'Sistema protetto automaticamente.',0
code db 'ERROR CODE: SECURITY_FAILURE',0

SECTION .text
GLOBAL security_failure
