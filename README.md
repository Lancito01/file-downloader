# File Downloader

A modern, elegant desktop application for downloading audio and video content using yt-dlp. Built with Tauri, Svelte, and Rust.

![File Downloader](https://img.shields.io/badge/Tauri-2.8+-blue) ![Svelte](https://img.shields.io/badge/Svelte-5.0+-orange) ![Rust](https://img.shields.io/badge/Rust-1.77+-red)

## 🚀 Quick Start

### Option 1: Easy Launch (Recommended)

**Windows PowerShell:**
```powershell
.\setup-and-run.ps1
```

**Windows Command Prompt:**
```cmd
run.bat
```

These scripts will:
- ✅ Check and configure your environment
- ✅ Auto-install yt-dlp on first run if missing
- ✅ Provide ffmpeg (bundled or system-installed)
- ✅ Load Visual Studio environment if installed
- ✅ Launch the app

### Option 2: Manual Setup

If the scripts don't work or you prefer manual setup:

1. **Install Prerequisites:**
   - ✅ [Node.js](https://nodejs.org/) (v18+)
   - ✅ [Rust](https://rustup.rs/)
   - ✅ [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
     - During installation, select "Desktop development with C++"
     - Include Windows 10/11 SDK

2. **Install yt-dlp (recommended - automatically checked on first run):**
   - If you prefer to pre-install: `python -m pip install -U --user yt-dlp`
   - Or use package manager: `brew install yt-dlp` (macOS) or `winget install -e --id yt-dlp.yt-dlp` (Windows)
   - The app will prompt you to install if missing

3. **Optional: Pre-install ffmpeg (app can use bundled version):**
   - macOS: `brew install ffmpeg`
   - Windows: `winget install -e --id Gyan.FFmpeg` or `Gyan.FFmpeg`
   - Linux: `sudo apt-get install ffmpeg`

4. **Install Node Dependencies:**
   ```bash
   npm install
   ```

5. **Launch:**
   ```bash
   npm run tauri dev
   ```

## 📦 How Dependencies Work

### yt-dlp (Required for downloads)
- **Strategy:** Always uses system-installed version from PATH
- **Why:** yt-dlp frequently updates and needs fresh versions for new sites
- **First Run:** App checks if yt-dlp is installed. If missing, prompts for auto-install via:
  - User-scope options first (no admin needed): `pipx`, `pip --user`
  - System installers if needed: `brew` (macOS), `winget` (Windows)
- **Updates:** On every launch, app checks if your yt-dlp is outdated and prompts to update
- **Manual Update:** `python -m pip install -U --user yt-dlp` or use your package manager

### ffmpeg (Required for video processing)
- **Strategy:** Try system-installed version first, fall back to bundled binaries
- **Why:** More stable than yt-dlp, infrequent updates
- **System ffmpeg:** If you have ffmpeg/ffprobe on PATH, the app will use it
- **Bundled ffmpeg:** If system version is missing, bundled version (included with app) is used
- **Manual Update:** Update via your package manager or download from https://ffmpeg.org

## 🔒 Download Blocking & Safety

The app enforces a strict policy:
- ✅ Downloads **only work** if yt-dlp is installed and ready
- 🔒 Download buttons are **disabled** and greyed out if dependencies are missing
- 📢 Clear warnings tell you what's needed and how to fix it
- 🎯 On first run, you're guided through dependency setup

## ⚠️ Troubleshooting

### "linker 'link.exe' not found"

**Cause:** Visual Studio Build Tools not installed or not in PATH.

**Solutions:**
1. **Install Visual Studio Build Tools:**
   - Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Select "Desktop development with C++"
   - Restart your computer after installation

2. **OR use the provided scripts:** `setup-and-run.ps1` or `run.bat` will configure the environment

3. **OR run from VS Developer Command Prompt:**
   - Open "x64 Native Tools Command Prompt for VS 2022" from Start Menu
   - Navigate to project folder
   - Run `npm run tauri dev`

### "cargo not found"

**Cause:** Rust not installed or not in PATH.

**Solution:**
1. Install Rust from https://rustup.rs/
2. Restart your terminal
3. Verify: `cargo --version`

### "Download buttons are disabled" or "Install Dependencies First"

**Cause:** yt-dlp is not installed.

**Solution:**
1. Look for an "Install yt-dlp" option in the Settings tab
2. Click "Auto-Install" and follow prompts
3. If auto-install fails:
   - Install Python from https://python.org
   - Then run: `python -m pip install -U --user yt-dlp`
   - Restart the app
4. If you prefer system install: `brew install yt-dlp` (macOS) or `winget install -e --id yt-dlp.yt-dlp` (Windows)
5. Verify: `yt-dlp --version` in terminal

### "yt-dlp is outdated" (Alert on launch)

**Solution:**
- Click "Update now" in the alert to auto-update
- Or manually update: `python -m pip install -U --user yt-dlp`
- Or use your package manager: `brew upgrade yt-dlp` (macOS)

### "ffmpeg/ffprobe not found" or Merging Fails

**Cause:** ffmpeg is not available (system or bundled version missing).

**What's happening:**
- The app checks for ffmpeg in your system PATH first (using `where ffmpeg` on Windows or `which ffmpeg` on macOS)
- If not found on PATH, it falls back to bundled ffmpeg in the app installation directory
- If neither is found, downloads that need video merging will fail

**Solution:**

1. **Verify ffmpeg is on your PATH:**
   - Windows: Open Command Prompt and type `where ffmpeg` - if it shows a path, ffmpeg is found
   - macOS: Open Terminal and type `which ffmpeg` - if it shows a path, ffmpeg is found
   
2. **If ffmpeg is NOT found on PATH, install it:**
   - **Windows:** 
     - Option A (Recommended): `winget install -e --id Gyan.FFmpeg`
     - Option B: Download from https://ffmpeg.org/download.html and add to PATH
   - **macOS:**
     - Option A (Recommended): `brew install ffmpeg`
     - Option B: Download from https://evermeet.cx/ffmpeg/
   
3. **After installing, restart the app** and try downloading again

4. **Still not working?**
   - Make sure you restarted the Command Prompt/Terminal after installing ffmpeg
   - On Windows, verify PATH by typing `echo %PATH%` in Command Prompt - you should see ffmpeg's directory
   - On macOS, verify PATH by typing `echo $PATH` in Terminal

**Technical Note:** The app prefers system ffmpeg over bundled ffmpeg. If you have ffmpeg installed but it's still not being found, the PATH may not be updated. Restart your terminal/command prompt to refresh PATH.


### Auto-Install Failed (Installation Error)

**When it happens:**
- First-time setup on fresh Windows/macOS
- No Python or package manager installed
- Missing admin permissions

**Solution:**
1. **Install Python** (if you don't have it):
   - https://www.python.org/downloads/
   - Make sure to check "Add Python to PATH" during installation
   
2. **Restart the app** and try again

3. **Manual installation** (fallback):
   - Windows: `winget install -e --id yt-dlp.yt-dlp --accept-source-agreements --accept-package-agreements`
   - macOS: `brew install yt-dlp`
   - Linux: `python3 -m pip install -U --user yt-dlp`

### First Compilation Takes Forever

**Normal:** Rust compilation takes 2-5 minutes on first run. Subsequent launches are fast.

### App Window Won't Open

**Try:**
1. Check if port 5173 is in use by another app
2. Restart your computer (applies Visual Studio installation)
3. Run from command prompt with administrator privileges
4. Check for errors in the terminal window

## ✨ Features

### 🎵 Single Downloads
- Download individual audio or video files
- Support for YouTube, SoundCloud, and many other platforms
- Customizable format and extension
- Optional metadata and artwork embedding
- Real-time download history with status tracking
- Search functionality (use `ytsearch:your query`)

### 📦 Bulk Downloads  
- Process multiple downloads sequentially
- Progress tracking for entire queue
- Individual status for each download
- Support for playlists and multiple URLs
- Visual feedback with progress bars

### ⚙️ Configurable Settings
- Set default download folder
- Configure default format preferences (audio/video)
- Toggle metadata embedding by default
- Settings persist across sessions

## 📖 Usage

### First Run

1. Launch the application
2. You'll be prompted to select a default download folder
3. Choose your preferred folder and click "Save"
4. You're ready to start downloading!

### Single Download

1. Navigate to the **Single Download** tab
2. Enter a URL or search term:
   - Direct URL: `https://youtube.com/watch?v=...`
   - YouTube search: `ytsearch:song name`
   - SoundCloud search: `scsearch:artist song`
3. Select format (Audio/Video)
4. Optionally specify an extension (mp3, mp4, etc.)
5. Toggle metadata embedding if desired
6. Click **Download Now**
7. Monitor progress in the history panel

### Bulk Download

1. Navigate to the **Bulk Download** tab
2. Enter multiple URLs or search terms (one per line)
   - You can also separate them with commas
3. Configure format and extension settings
4. Click **Start Bulk Download**
5. Watch as each item is processed sequentially
6. View detailed status for each download in the queue

### Settings

1. Navigate to the **Settings** tab
2. Change your download folder
3. Set default format preference
4. Toggle default metadata embedding
5. Click **Save All Settings** to apply changes

## 🎨 UI Features

- **Dark Theme**: Easy on the eyes with a modern dark interface
- **Real-time Status**: Status bar shows current operation and state
- **Color-coded Feedback**: 
  - 🟢 Green for success
  - 🔴 Red for errors
  - 🟡 Yellow for warnings/in-progress
  - ℹ️ Blue for info
- **Responsive Design**: Works on various screen sizes
- **Loading Indicators**: Clear visual feedback during operations

## 🛠️ Technology Stack

### Frontend
- **Svelte 5**: Modern, reactive UI framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Fast build tool and dev server
- **SCSS**: Enhanced CSS with variables and functions

### Backend
- **Tauri 2.8**: Lightweight Rust framework for desktop apps
- **Rust**: Systems programming language for performance
- **yt-dlp**: Powerful media downloader (external dependency)

### State Management
- **Svelte Stores**: Reactive state management
- **Tauri Plugin Store**: Persistent settings storage

## 📁 Project Structure

```
File Downloader/
├── src/                      # Frontend source
│   ├── lib/                  # Shared utilities and components
│   │   ├── assets/          # Icons, constants
│   │   ├── styles/          # Global SCSS styles
│   │   └── utils.ts         # Core utilities and stores
│   └── routes/              # Application pages
│       ├── tabs/            # Tab components
│       │   ├── Single.svelte   # Single download UI
│       │   ├── Bulk.svelte     # Bulk download UI
│       │   └── Settings.svelte # Settings UI
│       ├── Nav.svelte       # Navigation component
│       ├── StatusBar.svelte # Status display
│       └── +page.svelte     # Main app page
├── src-tauri/               # Backend source
│   ├── src/
│   │   ├── main.rs          # Main Tauri application
│   │   └── lib.rs           # Library code
│   ├── bin/                 # External executables
│   └── Cargo.toml           # Rust dependencies
├── setup-and-run.ps1        # Easy setup script (PowerShell)
├── run.bat                  # Easy launch script (Batch)
└── package.json             # Node dependencies
```

## 🔧 Development

### Available Scripts

```bash
# Run development server with hot reload
npm run dev

# Run Tauri app in development mode
npm run tauri dev

# Build frontend only
npm run build

# Build complete Tauri application
npm run tauri build

# Type check TypeScript and Svelte files
npm run check

# Watch mode for type checking
npm run check:watch
```

### Adding New Features

1. **Frontend Changes**: Edit files in `src/`
2. **Backend Changes**: Edit Rust files in `src-tauri/src/`
3. **Styling**: Modify SCSS in `src/lib/styles/` or component styles
4. **New Commands**: Add Tauri commands in `main.rs` and invoke from frontend

## 📝 License

This project is provided as-is for personal use.

## 🙏 Acknowledgments

- **yt-dlp**: https://github.com/yt-dlp/yt-dlp
- **Tauri**: https://tauri.app
- **Svelte**: https://svelte.dev

---

**Made with ❤️ for easy, organized downloads**
