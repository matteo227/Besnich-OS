BITS 32

SECTION .data

title db '[FILE ERROR]',0
msg1 db 'File non trovato.',0
code db 'ERROR: FILE_NOT_FOUND',0

SECTION .text
GLOBAL file_not_found
