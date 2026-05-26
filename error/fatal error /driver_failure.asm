BITS 32

SECTION .data

title db 'BESNICH OS DRIVER FAILURE',0
msg1 db 'Un driver critico ha smesso di funzionare.',0
msg2 db 'Riavvia il computer.',0
code db 'ERROR CODE: DRIVER_FAILURE',0

SECTION .text
GLOBAL driver_failure
