BITS 32

SECTION .data

title db 'BESNICH OS DISK FAILURE',0
msg1 db 'Errore critico disco.',0
msg2 db 'Impossibile leggere i dati.',0
code db 'ERROR CODE: DISK_FAILURE',0

SECTION .text
GLOBAL disk_failure
