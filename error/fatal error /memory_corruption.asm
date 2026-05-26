BITS 32

SECTION .data

title db 'BESNICH OS MEMORY FAILURE',0
msg1 db 'Memoria corrotta rilevata.',0
msg2 db 'Il sistema e stato fermato.',0
code db 'ERROR CODE: MEMORY_CORRUPTION',0

SECTION .text
GLOBAL memory_failure
