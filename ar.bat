@echo off
REM Wrapper for Argon Shell Script
REM Ensures we use the bash logic (scaffolding, etc) on Windows
REM Assumption: 'bash' (Git Bash) is in PATH

set SCRIPT_DIR=%~dp0
bash "%SCRIPT_DIR%ar.sh" %*
