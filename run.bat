@echo off
echo =====================================
echo   File Downloader - Quick Launch
echo =====================================
echo.

REM Check if VS environment is loaded
where link.exe >nul 2>&1
if errorlevel 1 (
    echo Loading Visual Studio environment...
    
    REM Try to find and load VS Developer Command Prompt
    if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" (
        call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=amd64 -host_arch=amd64
    ) else if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" (
        call "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=amd64 -host_arch=amd64
    ) else (
        echo Warning: Visual Studio environment not found
        echo The app may fail to compile.
        echo.
        pause
    )
)

echo Launching File Downloader...
echo This may take a few minutes on first run...
echo.

npm run tauri dev

pause
