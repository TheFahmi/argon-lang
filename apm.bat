@echo off
REM Argon Package Manager Windows Wrapper
REM Requires Git Bash or WSL

setlocal

REM Get the directory of this script
set SCRIPT_DIR=%~dp0

REM Check if bash is available
where bash >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: bash not found. Please install Git Bash or WSL.
    exit /b 1
)

REM Run the bash script with all arguments
bash "%SCRIPT_DIR%apm.sh" %*
