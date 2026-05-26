@echo off
title Besnich Decryptor

echo ============================
echo      BESNICH DECRYPTOR
echo ============================
echo.

set /p file=File da decriptare: 
set /p key=Chiave: 

if not exist "%file%" (
    echo File non trovato.
    pause
    exit
)

echo Decriptazione in corso...

certutil -decode "%file%" decrypted_output.bin >nul

echo.
echo File decriptato: decrypted_output.bin
echo Operazione completata.
pause
