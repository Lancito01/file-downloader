@echo off
echo =====================================
echo   File Downloader - Quick Launch
echo =====================================
echo.

REM Try VS 2022 Community first
if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" (
    echo Loading Visual Studio 2022 environment...
    call "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=amd64 -host_arch=amd64
    goto :launch
)

REM Try VS 2026 Community
if exist "C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat" (
    echo Loading Visual Studio 2026 environment...
    call "C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat" -arch=amd64 -host_arch=amd64
    goto :launch
)

REM Try Build Tools 2022
if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" (
    echo Loading Visual Studio Build Tools 2022 environment...
    call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=amd64 -host_arch=amd64
    goto :launch
)

echo.
echo Warning: Visual Studio environment not found
echo The app may fail to compile.
echo.
pause

:launch
echo.
echo Launching File Downloader...
echo This may take a few minutes on first run...
echo.

npm run tauri dev

pause
