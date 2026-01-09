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

#[derive(Default)]
struct NotepadApp {
    text: String,
    current_file: Option<String>,
    saved_text: String,
    font_size: f32,
    error_message: Option<String>,
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
            .set_file_name("untitled.txt")
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
        let file_name = self.current_file
            .as_ref()
            .and_then(|path| std::path::Path::new(path).file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("Untitled");
        
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
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update font size
        ctx.style_mut().text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(self.font_size),
        );

        // Update window title
        frame.set_window_title(&self.get_window_title());

        // Keyboard shortcuts
        let input = ctx.input(|i| i.clone());
        if input.key_pressed(egui::Key::S) && input.modifiers.ctrl {
            self.save_file();
        }
        if input.key_pressed(egui::Key::O) && input.modifiers.ctrl {
            self.open_file();
        }
        if input.key_pressed(egui::Key::N) && input.modifiers.ctrl {
            self.new_file();
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
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
                ui.menu_button("View", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Font Size:");
                        ui.add(egui::Slider::new(&mut self.font_size, 8.0..=32.0));
                    });
                });
            });
        });

        // Text editor
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.text);
        });

        // Show error messages
        if let Some(ref error) = self.error_message {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);
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
