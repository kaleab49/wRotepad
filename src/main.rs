use eframe::egui;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("wRotepad"),
        ..Default::default()
    };

    eframe::run_native(
        "wRotepad",
        options,
        Box::new(|_cc| Box::new(NotepadApp::new())),
    )
}

struct NotepadApp {
    text: String,
    current_file: Option<String>,
    saved_text: String,
    font_size: f32,
    error_message: Option<String>,
    text_editor_id: egui::Id, // Unique ID for the text editor to manage focus
    needs_focus: bool, // Track if we need to request focus on next frame
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            current_file: None,
            saved_text: String::new(),
            font_size: 14.0,
            error_message: None,
            text_editor_id: egui::Id::new("main_text_editor"),
            needs_focus: true, // Request focus on first frame
        }
    }
}

impl NotepadApp {
    fn new() -> Self {
        Self {
            font_size: 14.0,
            ..Default::default()
        }
    }

    fn new_file(&mut self) {
        self.text = String::new();
        self.current_file = None;
        self.saved_text = String::new();
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match fs::read_to_string(&path) {
                Ok(contents) => {
                    self.text = contents.clone();
                    self.saved_text = contents;
                    self.current_file = Some(path.to_string_lossy().to_string());
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to open file: {}", e));
                }
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.current_file {
            match fs::write(path, &self.text) {
                Ok(_) => {
                    self.saved_text = self.text.clone();
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to save file: {}", e));
                }
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("unnamed.txt")
            .save_file()
        {
            match fs::write(&path, &self.text) {
                Ok(_) => {
                    self.current_file = Some(path.to_string_lossy().to_string());
                    self.saved_text = self.text.clone();
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to save file: {}", e));
                }
            }
        }
    }

    fn has_unsaved_changes(&self) -> bool {
        self.text != self.saved_text
    }

    fn get_window_title(&self) -> String {
        let file_name = self
            .current_file
            .as_ref()
            .and_then(|path| std::path::Path::new(path).file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("Unnamed");

        if self.has_unsaved_changes() {
            format!("{}* - wRotepad", file_name)
        } else {
            format!("{} - wRotepad", file_name)
        }
    }

    fn get_line_count(&self) -> usize {
        if self.text.is_empty() {
            1
        } else {
            self.text.lines().count()
        }
    }

    fn get_word_count(&self) -> usize {
        if self.text.trim().is_empty() {
            0
        } else {
            self.text.split_whitespace().count()
        }
    }

    fn get_char_count(&self) -> usize {
        self.text.chars().count()
    }

    // Select all text in the editor
    // This function prepares the text for selection
    fn select_all(&mut self) {
        // In egui, selection is handled by the TextEdit widget itself
        // We'll add a visual indicator that select all was triggered
        // The actual selection happens in the UI layer
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update font size
        ctx.style_mut(|style| {
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(self.font_size),
            );
        });

        // Update window title
        ctx.send_viewport_cmd_to(
            egui::ViewportId::ROOT,
            egui::ViewportCommand::Title(self.get_window_title()),
        );

        // Handle keyboard shortcuts
        let input = ctx.input(|i| i.clone());
        
        // File shortcuts
        if input.key_pressed(egui::Key::S) && input.modifiers.ctrl {
            self.save_file();
        }
        if input.key_pressed(egui::Key::O) && input.modifiers.ctrl {
            self.open_file();
        }
        if input.key_pressed(egui::Key::N) && input.modifiers.ctrl {
            self.new_file();
        }
        
        // Edit shortcuts (Copy, Paste, Cut, Select All)
        // Note: Copy/Paste/Cut work automatically in egui TextEdit widgets
        // We add menu items for user visibility, but the shortcuts work natively
        if input.key_pressed(egui::Key::A) && input.modifiers.ctrl {
            self.select_all();
        }

        // Menu bar - separated header that doesn't accept text input
        // This panel is only for menu buttons, not for text editing
        egui::TopBottomPanel::top("menu_bar")
            .resizable(false) // Can't resize the menu bar
            .show(ctx, |ui| {
                // Menu bar doesn't accept keyboard focus - it's only for clicking
                egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New\tCtrl+N").clicked() {
                        self.new_file();
                    }
                    if ui.button("Open\tCtrl+O").clicked() {
                        self.open_file();
                    }
                    if ui.button("Save\tCtrl+S").clicked() {
                        self.save_file();
                    }
                    if ui.button("Save As").clicked() {
                        self.save_file_as();
                    }
                });
                
                // Edit menu with copy, paste, cut, and select all
                ui.menu_button("Edit", |ui| {
                    // Note: Copy (Ctrl+C), Paste (Ctrl+V), and Cut (Ctrl+X) 
                    // work automatically in the text editor - no code needed!
                    // These menu items are here for user reference
                    ui.label("Copy\tCtrl+C");
                    ui.label("Paste\tCtrl+V");
                    ui.label("Cut\tCtrl+X");
                    ui.separator();
                    if ui.button("Select All\tCtrl+A").clicked() {
                        self.select_all();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Font Size:");
                        ui.add(egui::Slider::new(&mut self.font_size, 8.0..=32.0));
                    });
                });
            });
        });

        // Text editor - fills entire body, completely separated from header
        // This is the only area where text input is accepted
        egui::CentralPanel::default()
            .frame(egui::Frame::none()) // No border, no background
            .show(ctx, |ui| {
                // Set minimum size to fill available space
                ui.set_min_size(ui.available_size());
                
                // Create the text editor widget with unique ID
                // This ensures it can always be focused and identified
                let text_edit = egui::TextEdit::multiline(&mut self.text)
                    .id(self.text_editor_id) // Use unique ID for focus management
                    .desired_width(f32::INFINITY)
                    .desired_rows(usize::MAX)
                    .frame(false); // Remove the text edit frame/border
                
                // Add the text editor to fill entire available space
                let response = ui.add_sized(ui.available_size(), text_edit);
                
                // Always ensure the text editor has focus
                // This prevents typing from going to the header
                if self.needs_focus || !response.has_focus() {
                    // Request focus for the text editor
                    response.request_focus();
                    self.needs_focus = false; // Only request once on startup
                }
                
                // If user clicks in the text area, ensure it gets focus
                if response.clicked() && !response.has_focus() {
                    response.request_focus();
                }
                
                // Handle Select All shortcut only when text editor has focus
                if response.has_focus() {
                    let input = ctx.input(|i| i.clone());
                    if input.key_pressed(egui::Key::A) && input.modifiers.ctrl {
                        // Select all is handled automatically by egui when focused
                        response.request_focus();
                    }
                }
            });

        // Show error messages
        if let Some(error) = self.error_message.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&error);
                    if ui.button("OK").clicked() {
                        self.error_message = None;
                    }
                });
        }

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Lines: {}", self.get_line_count()));
                ui.separator();
                ui.label(format!("Words: {}", self.get_word_count()));
                ui.separator();
                ui.label(format!("Characters: {}", self.get_char_count()));
            });
        });
    }
}
