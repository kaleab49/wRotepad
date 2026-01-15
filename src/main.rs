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
        Box::new(|_cc| Box::new(NotepadApp::default())),
    )
}

struct NotepadApp {
    text: String,
    saved_text: String,
    current_file: Option<String>,
    font_size: f32,
    error: Option<String>,
    editor_id: egui::Id,
    request_focus: bool,
    // Find feature state
    find_open: bool,
    find_text: String,
    find_matches: Vec<usize>, // Positions where matches are found
    current_match_index: usize, // Which match we're currently viewing
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            saved_text: String::new(),
            current_file: None,
            font_size: 14.0,
            error: None,
            editor_id: egui::Id::new("editor"),
            request_focus: true,
            find_open: false,
            find_text: String::new(),
            find_matches: Vec::new(),
            current_match_index: 0,
        }
    }
}

impl NotepadApp {
    /* ---------- File actions ---------- */

    fn new_file(&mut self) {
        self.text.clear();
        self.saved_text.clear();
        self.current_file = None;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match fs::read_to_string(&path) {
                Ok(data) => {
                    self.text = data.clone();
                    self.saved_text = data;
                    self.current_file = Some(path.to_string_lossy().into());
                }
                Err(e) => self.error = Some(e.to_string()),
            }
        }
    }

    fn save(&mut self) {
        match &self.current_file {
            Some(path) => {
                if let Err(e) = fs::write(path, &self.text) {
                    self.error = Some(e.to_string());
                } else {
                    self.saved_text = self.text.clone();
                }
            }
            None => self.save_as(),
        }
    }

    fn save_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            if let Err(e) = fs::write(&path, &self.text) {
                self.error = Some(e.to_string());
            } else {
                self.current_file = Some(path.to_string_lossy().into());
                self.saved_text = self.text.clone();
            }
        }
    }

    /* ---------- State helpers ---------- */

    fn dirty(&self) -> bool {
        self.text != self.saved_text
    }

    fn title(&self) -> String {
        let name = self
            .current_file
            .as_deref()
            .and_then(|p| std::path::Path::new(p).file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");

        if self.dirty() {
            format!("{}* - wRotepad", name)
        } else {
            format!("{} - wRotepad", name)
        }
    }

    /* ---------- Find feature ---------- */

    fn open_find(&mut self) {
        self.find_open = true;
        self.find_text.clear();
        self.find_matches.clear();
        self.current_match_index = 0;
    }

    fn close_find(&mut self) {
        self.find_open = false;
        self.find_text.clear();
        self.find_matches.clear();
        self.current_match_index = 0;
    }

    // Search for all matches of the find text in the document
    fn search_matches(&mut self) {
        self.find_matches.clear();
        
        // If search text is empty, no matches
        if self.find_text.is_empty() {
            return;
        }

        // Find all occurrences of the search text (case-sensitive)
        let search_text = &self.find_text;
        let mut start = 0;
        
        while let Some(pos) = self.text[start..].find(search_text) {
            let absolute_pos = start + pos;
            self.find_matches.push(absolute_pos);
            start = absolute_pos + search_text.len();
        }
    }

    // Navigate to next match
    fn find_next(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }
        self.current_match_index = (self.current_match_index + 1) % self.find_matches.len();
    }

    // Navigate to previous match
    fn find_previous(&mut self) {
        if self.find_matches.is_empty() {
            return;
        }
        if self.current_match_index == 0 {
            self.current_match_index = self.find_matches.len() - 1;
        } else {
            self.current_match_index -= 1;
        }
    }
}

/* ================== egui App ================== */

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        /* ---------- Global style ---------- */

        ctx.style_mut(|s| {
            s.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(self.font_size),
            );
        });

        ctx.send_viewport_cmd(
            egui::ViewportCommand::Title(self.title()),
        );

        /* ---------- Shortcuts ---------- */

        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::O) {
                self.open_file();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::N) {
                self.new_file();
            }
            // Open find dialog with Ctrl+F
            if i.modifiers.ctrl && i.key_pressed(egui::Key::F) {
                self.open_find();
            }
            // Close find dialog with Escape
            if i.key_pressed(egui::Key::Escape) && self.find_open {
                self.close_find();
            }
        });

        /* ---------- Menu bar ---------- */

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save();
                        ui.close_menu();
                    }
                    if ui.button("Save As").clicked() {
                        self.save_as();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Find\tCtrl+F").clicked() {
                        self.open_find();
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.add(egui::Slider::new(&mut self.font_size, 8.0..=32.0)
                        .text("Font size"));
                });
            });
        });

        /* ---------- Editor ---------- */

        egui::CentralPanel::default().show(ctx, |ui| {
            let editor = egui::TextEdit::multiline(&mut self.text)
                .id(self.editor_id)
                .frame(false)
                .desired_width(f32::INFINITY);

            let response = ui.add_sized(ui.available_size(), editor);

            if self.request_focus {
                response.request_focus();
                self.request_focus = false;
            }
        });

        /* ---------- Status bar ---------- */

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Lines: {}", self.text.lines().count().max(1)));
                ui.separator();
                ui.label(format!("Words: {}", self.text.split_whitespace().count()));
                ui.separator();
                ui.label(format!("Chars: {}", self.text.chars().count()));
            });
        });

        /* ---------- Find dialog (VSCode-style) ---------- */

        if self.find_open {
            // Position the find bar at the top right, like VSCode
            // We'll use a fixed width and position it relative to screen
            egui::Window::new("Find")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
                .fixed_size([400.0, 80.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // Search text input with unique ID for focus management
                        let find_input_id = egui::Id::new("find_input");
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.find_text)
                                .id(find_input_id)
                        );
                        
                        // Auto-focus the search box when find dialog opens (first time)
                        if self.find_text.is_empty() {
                            response.request_focus();
                        }
                        
                        // Update matches when search text changes
                        if response.changed() {
                            self.search_matches();
                            self.current_match_index = 0;
                        }

                        // Handle Enter key for navigation (next match)
                        if response.has_focus() {
                            ctx.input(|i| {
                                if i.key_pressed(egui::Key::Enter) {
                                    if i.modifiers.shift {
                                        self.find_previous();
                                    } else {
                                        self.find_next();
                                    }
                                }
                            });
                        }

                        // Up arrow button (previous match)
                        if ui.button("▲").clicked() {
                            self.find_previous();
                        }
                        
                        // Down arrow button (next match)
                        if ui.button("▼").clicked() {
                            self.find_next();
                        }

                        // Close button (X)
                        if ui.button("✕").clicked() {
                            self.close_find();
                        }
                    });

                    // Show match count or "No match" message
                    if !self.find_text.is_empty() {
                        if self.find_matches.is_empty() {
                            // Show "No match" in red
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.colored_label(egui::Color32::RED, "No match");
                            });
                        } else {
                            // Show match count (e.g., "1 of 5")
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                let match_text = if self.find_matches.len() > 0 {
                                    format!("{} of {}", 
                                        self.current_match_index + 1, 
                                        self.find_matches.len())
                                } else {
                                    String::new()
                                };
                                ui.label(match_text);
                            });
                        }
                    }
                });
        }

        /* ---------- Error dialog ---------- */

        if let Some(err) = self.error.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&err);
                    if ui.button("OK").clicked() {
                        self.error = None;
                    }
                });
        }
    }
}
