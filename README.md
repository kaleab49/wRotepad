# wRotepad

A simple notepad clone written in Rust using egui.

## Features

- Simple text editing with multiline support
- File operations: New, Open, Save, Save As
- Keyboard shortcuts (Ctrl+N, Ctrl+O, Ctrl+S)
- Dynamic window title showing current file name
- Unsaved changes indicator (* in window title)
- Status bar with line, word, and character count
- Adjustable font size (8-32pt)
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
- **View Menu**: Adjust font size with slider
- **Keyboard Shortcuts**:
  - `Ctrl+N`: New file
  - `Ctrl+O`: Open file
  - `Ctrl+S`: Save file
- **Status Bar**: Shows line count, word count, and character count
- Type directly in the text area to edit
