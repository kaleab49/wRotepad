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

#[derive(Default)]
struct NotepadApp {
    text: String,
    current_file: Option<String>,
}

impl NotepadApp {
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(contents) = fs::read_to_string(&path) {
                self.text = contents;
                self.current_file = Some(path.to_string_lossy().to_string());
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.current_file {
            if let Err(e) = fs::write(path, &self.text) {
                eprintln!("Error saving file: {}", e);
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
            if let Err(e) = fs::write(&path, &self.text) {
                eprintln!("Error saving file: {}", e);
            } else {
                self.current_file = Some(path.to_string_lossy().to_string());
            }
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.open_file();
                    }
                    if ui.button("Save").clicked() {
                        self.save_file();
                    }
                    if ui.button("Save As").clicked() {
                        self.save_file_as();
                    }
                });
            });
        });

        // Text editor
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.text);
        });
    }
}
