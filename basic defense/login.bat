@echo off
setlocal enabledelayedexpansion

set USER_FILE=user.dat

if not exist %USER_FILE% (
    set /p NEWPASS=Set your password:
    echo %NEWPASS%> %USER_FILE%
    exit /b 0
)

set /p PASS=Password:
set /p STORED=<%USER_FILE%

if "%PASS%"=="%STORED%" (
    exit /b 0
) else (
    exit /b 1
)
