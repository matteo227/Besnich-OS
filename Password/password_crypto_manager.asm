BITS 32

SECTION .text
GLOBAL encrypt_password

encrypt_password:

    call sam_password_encrypt
    call e2e_password_encrypt
    call aes256_password_encrypt

    ret
