# wRotepad

A simple notepad clone written in Rust using egui.

## Features

- Simple text editing with multiline support
- File operations: New, Open, Save, Save As
- Keyboard shortcuts (Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+F)
- Dynamic window title showing current file name
- Unsaved changes indicator (* in window title)
- Status bar with line, word, and character count
- Adjustable font size (8-32pt)
- VSCode-style find feature with search bar and navigation
- User-friendly error dialogs

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Usage

- **File Menu**: New, Open, Save, Save As
- **Edit Menu**: Find (opens search dialog)
- **View Menu**: Adjust font size with slider
- **Keyboard Shortcuts**:
  - `Ctrl+N`: New file
  - `Ctrl+O`: Open file
  - `Ctrl+S`: Save file
  - `Ctrl+F`: Open find dialog
  - `Escape`: Close find dialog
  - `Enter`: Next match (in find dialog)
  - `Shift+Enter`: Previous match (in find dialog)
- **Find Feature**:
  - Press `Ctrl+F` to open the search bar
  - Type to search in real-time
  - Use arrow buttons (▲ ▼) or Enter to navigate matches
  - Shows "No match" in red when search text not found
  - Displays match count (e.g., "1 of 5")
- **Status Bar**: Shows line count, word count, and character count
- Type directly in the text area to edit
