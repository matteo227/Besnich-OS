@echo off
setlocal enabledelayedexpansion

set BLOCKED_IPS=192.168.1.10 10.0.0.5
set BLOCKED_PORTS=23 445 3389

set IP=%1
set PORT=%2

for %%i in (%BLOCKED_IPS%) do (
    if "%%i"=="%IP%" exit /b 1
)

for %%p in (%BLOCKED_PORTS%) do (
    if "%%p"=="%PORT%" exit /b 1
)

exit /b 0
