# File Downloader - Setup and Run Script
# This script configures the environment and launches the app

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  File Downloader - Setup & Launch  " -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Add Cargo to PATH
Write-Host "[1/4] Configuring Rust environment..." -ForegroundColor Yellow
$env:Path += ";$env:USERPROFILE\.cargo\bin"

# Verify Rust is installed
try {
    $rustVersion = & cargo --version 2>&1
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust not found. Please install from https://rustup.rs" -ForegroundColor Red
    exit 1
}

# Step 2: Check for Visual Studio Build Tools
Write-Host ""
Write-Host "[2/4] Checking for Visual Studio Build Tools..." -ForegroundColor Yellow

# Try to find and load VS environment
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vsPath = $null

if (Test-Path $vsWhere) {
    $vsPath = & $vsWhere -latest -property installationPath 2>$null
}

if ($vsPath -and (Test-Path "$vsPath\VC\Auxiliary\Build\vcvars64.bat")) {
    Write-Host "✅ Found Visual Studio at: $vsPath" -ForegroundColor Green
    
    # Load VS environment by calling vcvars and capturing output
    $vcvarsPath = "$vsPath\VC\Auxiliary\Build\vcvars64.bat"
    
    Write-Host "   Loading Visual Studio environment..." -ForegroundColor Gray
    
    # Create temp batch file
    $tempBatch = "$env:TEMP\load_vs_env_$PID.bat"
    @"
@echo off
call "$vcvarsPath" > nul 2>&1
if errorlevel 1 exit /b 1
set
"@ | Out-File -FilePath $tempBatch -Encoding ASCII
    
    # Execute and parse environment
    $envVars = & cmd /c $tempBatch 2>&1
    Remove-Item $tempBatch -ErrorAction SilentlyContinue
    
    if ($LASTEXITCODE -eq 0) {
        foreach ($line in $envVars) {
            if ($line -match '^([^=]+)=(.*)$') {
                $varName = $matches[1]
                $varValue = $matches[2]
                if ($varName -eq "PATH" -or $varName -eq "LIB" -or $varName -eq "INCLUDE") {
                    Set-Item -Path "env:$varName" -Value $varValue
                }
            }
        }
        Write-Host "   ✅ Visual Studio environment loaded" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  Warning: Could not load VS environment" -ForegroundColor Yellow
        Write-Host "   The app may fail to compile." -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ Visual Studio Build Tools not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "   The app needs Visual Studio Build Tools to compile." -ForegroundColor Yellow
    Write-Host "   Please install from:" -ForegroundColor Yellow
    Write-Host "   https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "   During installation, select:" -ForegroundColor Yellow
    Write-Host "   - Desktop development with C++" -ForegroundColor Yellow
    Write-Host "   - Windows 10/11 SDK" -ForegroundColor Yellow
    Write-Host ""
    
    $response = Read-Host "Do you want to continue anyway? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        exit 1
    }
}

# Step 3: Verify yt-dlp.exe exists
Write-Host ""
Write-Host "[3/4] Checking for yt-dlp.exe..." -ForegroundColor Yellow

$ytdlpPath = ".\src-tauri\bin\yt-dlp.exe"
if (Test-Path $ytdlpPath) {
    Write-Host "✅ yt-dlp.exe found" -ForegroundColor Green
} else {
    Write-Host "❌ yt-dlp.exe not found at $ytdlpPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "   Please download yt-dlp.exe from:" -ForegroundColor Yellow
    Write-Host "   https://github.com/yt-dlp/yt-dlp/releases/latest" -ForegroundColor Cyan
    Write-Host "   And place it in: src-tauri\bin\yt-dlp.exe" -ForegroundColor Yellow
    Write-Host ""
    
    $response = Read-Host "Do you want to download it now? (Y/n)"
    if ($response -ne "n" -and $response -ne "N") {
        Write-Host "   Downloading yt-dlp.exe..." -ForegroundColor Gray
        
        # Create bin directory if it doesn't exist
        $binDir = ".\src-tauri\bin"
        if (-not (Test-Path $binDir)) {
            New-Item -ItemType Directory -Path $binDir | Out-Null
        }
        
        # Download latest yt-dlp
        try {
            Invoke-WebRequest -Uri "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe" -OutFile $ytdlpPath
            Write-Host "   ✅ Downloaded yt-dlp.exe" -ForegroundColor Green
        } catch {
            Write-Host "   ❌ Failed to download: $_" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "   ⚠️  Continuing without yt-dlp - downloads will not work!" -ForegroundColor Yellow
    }
}

# Step 4: Launch the app
Write-Host ""
Write-Host "[4/4] Launching File Downloader..." -ForegroundColor Yellow
Write-Host ""
Write-Host "This may take a while on first run (compiling Rust code)..." -ForegroundColor Gray
Write-Host "The app window will open when ready." -ForegroundColor Gray
Write-Host ""

try {
    npm run tauri dev
} catch {
    Write-Host ""
    Write-Host "❌ Failed to launch: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Common fixes:" -ForegroundColor Yellow
    Write-Host "1. Restart your terminal/PowerShell" -ForegroundColor Yellow
    Write-Host "2. Restart your computer (to apply Visual Studio installation)" -ForegroundColor Yellow
    Write-Host "3. Re-run this script" -ForegroundColor Yellow
    Write-Host ""
    Read-Host "Press Enter to exit"
    exit 1
}
