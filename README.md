# KiAssist

KiCAD AI Assistance - A cross-platform desktop application for AI-powered KiCAD design assistance.

## Features

- **Cross-Platform UI**: Built with Tauri + Vue for Windows, macOS, and Linux
- **AI-Powered Chat**: Interactive chat interface powered by Google Gemini LLM
  - Support for multiple Gemini models (1.5 Flash, 1.5 Pro, 1.5 Flash 8B)
  - Automatic API key detection from environment variables
  - Secure API key storage for seamless experience
  - Real-time responses with loading indicators
- **KiCAD Integration**: Direct connection to running KiCAD instances
- **Modern Design**: Professional UI with light/dark mode support
- **Python Integration**: Includes Python utilities package for KiCAD integration
- **Continuous Integration**: Automated builds for all major platforms

## Getting Started

### Prerequisites

- Node.js 20+
- Rust (latest stable)
- Python 3.8+
- Google Gemini API key (get one at [Google AI Studio](https://makersuite.google.com/app/apikey))
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `protobuf-compiler`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual Studio C++ Build Tools

### Configuring Gemini API

KiAssist can be configured with your Gemini API key in two ways:

**Option 1: Environment Variable (Recommended for Development)**
```bash
export GEMINI_API_KEY="your-api-key-here"
npm run tauri dev
```

**Option 2: Interactive Prompt**
- Launch the application without setting the environment variable
- A modal will prompt you to enter your API key
- The key will be securely stored for future sessions
- You can update the key anytime using the settings button (⚙️) in the chat header

## Development

### Setup

```bash
# Install Node dependencies
npm install

# Install Python package
cd python-lib
pip install -e .
cd ..
```

### Running in Development

```bash
npm run tauri dev
```

### Building for Production

```bash
npm run tauri build
```

Build artifacts will be available in `src-tauri/target/release/bundle/`.

## Project Structure

```
KiAssist/
├── src/                    # Vue frontend source
│   ├── components/        # Vue components
│   │   ├── ChatBox.vue   # AI chat interface with Gemini integration
│   │   └── KiCadInstanceSelector.vue  # KiCAD instance selection
│   ├── App.vue           # Main application component
│   └── main.ts           # Application entry point
├── src-tauri/             # Tauri backend
│   └── src/
│       ├── lib.rs        # Rust backend with Tauri commands
│       ├── gemini.rs     # Gemini API integration
│       ├── api_key.rs    # API key management and persistence
│       └── kicad_ipc.rs  # KiCAD IPC communication
├── python-lib/            # Python utilities package
│   └── kiassist_utils/
└── .github/workflows/     # CI/CD pipelines
```

## Using Gemini AI Chat

1. **Start the Application**: Launch KiAssist with `npm run tauri dev` or run the built application
2. **Configure API Key**: If not set via environment variable, enter your Gemini API key when prompted
3. **Select Model**: Choose your preferred Gemini model from the dropdown in the chat header:
   - **Gemini 1.5 Flash**: Fast and efficient for quick responses (1M token context)
   - **Gemini 1.5 Pro**: Advanced capabilities for complex queries (2M token context)
   - **Gemini 1.5 Flash 8B**: High volume, low latency model (1M token context)
4. **Start Chatting**: Type your questions about KiCAD or PCB design and get AI-powered responses
5. **Manage Settings**: Click the settings button (⚙️) to update your API key at any time

## Python Package

The `kiassist-utils` package provides utility functions for message processing and KiCAD project validation. It will be extended to integrate with KiCAD IPC APIs.

### Usage

```python
from kiassist_utils import process_message, validate_kicad_project

# Process messages
result = process_message("Hello, KiCAD!")

# Validate KiCAD projects
is_valid = validate_kicad_project("/path/to/project.kicad_pro")
```

## CI/CD

GitHub Actions workflows automatically build the application for:
- Ubuntu Linux (AppImage, DEB, RPM)
- macOS (DMG, App Bundle)
- Windows (MSI, EXE)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

TBD
