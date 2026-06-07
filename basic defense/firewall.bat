@echo off
setlocal enabledelayedexpansion

set WHITELIST_IPS=192.168.1.50
set BLOCKED_IPS=192.168.1.10 10.0.0.5
set BLOCKED_PORTS=23 135 139 445 3389
set ALLOW_PORTS=80 443 22

set IP=%1
set PORT=%2

set ALLOWED=0

for %%w in (%WHITELIST_IPS%) do (
    if "%%w"=="%IP%" set ALLOWED=1
)

if %ALLOWED%==1 exit /b 0

for %%i in (%BLOCKED_IPS%) do (
    if "%%i"=="%IP%" exit /b 1
)

for %%p in (%BLOCKED_PORTS%) do (
    if "%%p"=="%PORT%" exit /b 1
)

set PORT_OK=0
for %%a in (%ALLOW_PORTS%) do (
    if "%%a"=="%PORT%" set PORT_OK=1
)

if %PORT_OK%==1 exit /b 0

exit /b 1
