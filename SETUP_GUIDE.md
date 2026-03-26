# File Downloader - Complete Setup Guide

This guide will help you get the File Downloader app running on your Windows machine.

## TL;DR - Quick Start

**Choose one method:**

### Method 1: Automated Setup (Easiest)
```powershell
.\setup-and-run.ps1
```
The script will guide you through everything automatically.

### Method 2: Simple Launch
```cmd
run.bat
```
Works if you already have everything installed.

---

## Detailed Setup (If Scripts Don't Work)

### Step 1: Install Node.js

✅ **Check if you have it:**
```bash
node --version
```

❌ **If you don't have it:**
1. Download from: https://nodejs.org/
2. Install the LTS version
3. Restart your terminal
4. Verify: `node --version` should show v18 or higher

---

### Step 2: Install Rust

✅ **Check if you have it:**
```bash
cargo --version
```

❌ **If you don't have it:**
1. Go to: https://rustup.rs/
2. Download and run `rustup-init.exe`
3. Press `1` to proceed with standard installation
4. Wait for installation to complete
5. **Restart your computer** (important!)
6. Open a new terminal
7. Verify: `cargo --version` should show 1.77 or higher

---

### Step 3: Install Visual Studio Build Tools

This is **THE MOST IMPORTANT STEP** for Windows users. Without this, Rust cannot compile!

✅ **Check if you have it:**
```bash
link.exe
```
If you see "Microsoft (R) Incremental Linker", you're good!

❌ **If you don't have it:**

1. **Download the installer:**
   - Go to: https://visualstudio.microsoft.com/downloads/
   - Scroll down to "Tools for Visual Studio"
   - Download "Build Tools for Visual Studio 2022"

2. **Run the installer:**
   - It will download and launch the Visual Studio Installer
   - Wait for it to load

3. **Select components:**
   - ✅ Check "Desktop development with C++"
   - Make sure these are included (should be automatic):
     - MSVC v143 - VS 2022 C++ x64/x86 build tools
     - Windows 10/11 SDK

4. **Install:**
   - Click "Install"
   - This will take 5-15 minutes and download ~7GB
   - **Restart your computer after installation**

5. **Verify installation:**
   - Open "Start Menu"
   - Search for "x64 Native Tools Command Prompt for VS 2022"
   - If you see it, the installation succeeded!

---

### Step 4: Install Project Dependencies

Navigate to the project folder:
```bash
cd "D:\Andy\Coding\File Downloader"
npm install
```

This installs all the JavaScript/TypeScript dependencies.

---

### Step 5: Install yt-dlp (system command)

The app uses the system-installed `yt-dlp` command (not a bundled exe).

✅ **Check if you already have it:**
```powershell
yt-dlp --version
```

❌ **If it's missing, install it:**

**Option A: Automated (recommended)**
```powershell
# Run the setup script, it will prompt to install yt-dlp
.\setup-and-run.ps1
```

**Option B: Manual**
- Windows: `winget install -e --id yt-dlp.yt-dlp`
- Cross-platform: `python -m pip install -U --user yt-dlp`

After install, restart your terminal and re-run `yt-dlp --version`.

---

### Step 6: Install ffmpeg/ffprobe (system commands)

The app uses ffmpeg/ffprobe for merging and audio extraction.

✅ **Check if you already have them:**
```powershell
ffmpeg -version
ffprobe -version
```

❌ **If they're missing, install them:**

**Option A: Automated (recommended)**
```powershell
# Run the setup script, it will prompt to install ffmpeg/ffprobe
.\setup-and-run.ps1
```

**Option B: Manual**
- Windows: `winget install -e --id Gyan.FFmpeg`
- macOS: `brew install ffmpeg`

After install, restart your terminal and re-run `ffmpeg -version`.

---

### Step 7: Launch the App

You have three options:

#### Option A: Use setup script (recommended)
```powershell
.\setup-and-run.ps1
```

#### Option B: Use batch file
```cmd
run.bat
```

#### Option C: Use VS Developer Command Prompt
1. Open Start Menu
2. Search for "x64 Native Tools Command Prompt for VS 2022"
3. Navigate to project: `cd "D:\Andy\Coding\File Downloader"`
4. Run: `npm run tauri dev`

#### Option D: Regular terminal (if VS is in PATH)
```bash
npm run tauri dev
```

**First launch will take 2-5 minutes** as Rust compiles the backend. Subsequent launches are fast!

---

## Common Issues & Solutions

### "linker 'link.exe' not found"

**Cause:** Visual Studio Build Tools not installed or not in PATH

**Solutions:**
1. ✅ **Best:** Use `setup-and-run.ps1` or `run.bat` - they load the VS environment automatically
2. ✅ **Good:** Launch from "x64 Native Tools Command Prompt for VS 2022"
3. ✅ **Alternative:** Restart your computer after installing VS Build Tools
4. ✅ **Last resort:** Manually add to PATH:
   ```
   C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\[version]\bin\Hostx64\x64
   ```

### "cargo: command not found"

**Solutions:**
1. Restart your terminal/PowerShell
2. Restart your computer
3. Verify Rust is installed: Try opening a NEW terminal window
4. Manually add to PATH: `$env:Path += ";$env:USERPROFILE\.cargo\bin"`

### "npm: command not found"

**Solution:** Install Node.js from https://nodejs.org/

### Port 5173 already in use

**Solution:**
1. Close any other Vite/development servers
2. Kill the process using the port:
   ```bash
   Get-Process | Where-Object {$_.ProcessName -like "*vite*"} | Stop-Process
   ```

### App compiles but window doesn't appear

**Solutions:**
1. Check for errors in the terminal
2. Try running as administrator
3. Disable antivirus temporarily
4. Check Windows Firewall isn't blocking it

### "yt-dlp not found" or downloads fail

**Solution:**
1. Verify `yt-dlp --version` works in your terminal.
2. If missing, install with `winget` or `python -m pip install -U --user yt-dlp`.
3. Restart your terminal and relaunch the app.

### "ffmpeg/ffprobe not found" or downloads fail

**Solution:**
1. Verify `ffmpeg -version` and `ffprobe -version` work in your terminal.
2. If missing, install with `winget install -e --id Gyan.FFmpeg` or `brew install ffmpeg`.
3. Restart your terminal and relaunch the app.

---

## Still Having Issues?

### Check Prerequisites Checklist:
- ✅ Node.js installed (`node --version` works)
- ✅ Rust installed (`cargo --version` works)
- ✅ Visual Studio Build Tools installed (can find "x64 Native Tools Command Prompt")
- ✅ Computer restarted after installations
- ✅ yt-dlp available on PATH (`yt-dlp --version`)
- ✅ ffmpeg/ffprobe available on PATH (`ffmpeg -version`, `ffprobe -version`)
- ✅ Dependencies installed (`npm install` completed)

### Debug Steps:
1. **Try the setup script first:**
   ```powershell
   .\setup-and-run.ps1
   ```
   It will check everything and give specific error messages.

2. **Check individual components:**
   ```bash
   node --version
   cargo --version
   link.exe
   yt-dlp --version
   ffmpeg -version
   ffprobe -version
   ```

3. **Try compiling Rust directly:**
   ```bash
   cd src-tauri
   cargo build
   ```
   This will show specific Rust compilation errors.

4. **Check npm dependencies:**
   ```bash
   npm run check
   ```

### Last Resort:
1. Restart your computer
2. Open "x64 Native Tools Command Prompt for VS 2022" from Start Menu
3. Navigate to project folder
4. Run `npm run tauri dev`

This loads the VS environment automatically and should work.

---

## What Happens on First Launch?

When you run the app for the first time:

1. **Rust Compilation (2-5 minutes):**
   - You'll see lots of "Compiling..." messages
   - This downloads and compiles Rust dependencies
   - Only happens ONCE

2. **Vite Dev Server:**
   - Frontend development server starts
   - Usually takes 5-10 seconds

3. **App Window Opens:**
   - The File Downloader window appears
   - You'll see the first-run setup screen

4. **First-Run Setup:**
   - Click "Select Music folder" to choose download location
   - Click "Save"
   - You're ready to download!

**Subsequent launches:** Only take 10-20 seconds!

---

## Success Indicators

You'll know everything is working when:

✅ Terminal shows "Tauri app is starting..."
✅ You see "Compiled successfully" messages
✅ The File Downloader window opens
✅ You see the three tabs: Single, Bulk, Settings
✅ Status bar at bottom shows "Idle 🌙"

---

## Need More Help?

Check the main README.md for:
- Feature descriptions
- Usage instructions
- Development commands
- Project structure

---

**Happy downloading! 🎵**
