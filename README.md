# File Downloader

A modern, elegant desktop application for downloading audio and video content using yt-dlp. Built with Tauri, Svelte, and Rust.

![File Downloader](https://img.shields.io/badge/Tauri-2.8+-blue) ![Svelte](https://img.shields.io/badge/Svelte-5.0+-orange) ![Rust](https://img.shields.io/badge/Rust-1.77+-red)

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

## 🚀 Getting Started

### Prerequisites

- Node.js (v18 or higher)
- npm or pnpm
- Rust (v1.77 or higher) - for building from source
- yt-dlp executable (should be bundled in `src-tauri/bin/yt-dlp.exe`)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd "File Downloader"
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Ensure yt-dlp is present**
   - Download yt-dlp.exe from https://github.com/yt-dlp/yt-dlp/releases
   - Place it in `src-tauri/bin/yt-dlp.exe`

4. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

5. **Build for production**
   ```bash
   npm run tauri build
   ```

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
- **Tailwind CSS**: Utility-first CSS (for some components)

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
│   │   └── yt-dlp.exe      # yt-dlp downloader
│   └── Cargo.toml           # Rust dependencies
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

## 🐛 Troubleshooting

### Downloads Not Working

- **Check yt-dlp**: Ensure `yt-dlp.exe` is in `src-tauri/bin/`
- **Update yt-dlp**: Download the latest version from GitHub releases
- **Check Folder**: Verify the download folder exists and is writable
- **URL Format**: Try using direct URLs instead of search terms

### Build Errors

- **Clear Cache**: Delete `node_modules` and `build` directories, then reinstall
- **Rust Errors**: Ensure Rust toolchain is installed (`rustup update`)
- **Port Conflicts**: If dev server fails, check if port 5173 is in use

### Settings Not Saving

- **First Run**: Complete the first-run setup before using other features
- **Permissions**: Ensure the app has write permissions to its config directory

## 📝 License

This project is provided as-is for personal use.

## 🙏 Acknowledgments

- **yt-dlp**: https://github.com/yt-dlp/yt-dlp
- **Tauri**: https://tauri.app
- **Svelte**: https://svelte.dev

---

**Made with ❤️ for easy, organized downloads**
