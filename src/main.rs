use std::collections::BTreeMap;
use std::path::PathBuf;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([820.0, 520.0])
            .with_min_inner_size([600.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "File Type Counter",
        options,
        Box::new(|_cc| Ok(Box::<FileTypeCounter>::default())),
    )
}

#[derive(Default)]
struct FileTypeCounter {
    files: Vec<PathBuf>,
}

impl FileTypeCounter {
    fn add_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped_files = ctx.input(|input| input.raw.dropped_files.clone());
        for dropped_file in dropped_files {
            if let Some(path) = dropped_file.path {
                if path.is_file() {
                    self.files.push(path);
                }
            }
        }
    }

    fn counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for file in &self.files {
            let extension = file
                .extension()
                .and_then(|ext| ext.to_str())
                .filter(|ext| !ext.is_empty())
                .map(|ext| format!(".{}", ext.to_lowercase()))
                .unwrap_or_else(|| "[No extension]".to_owned());
            *counts.entry(extension).or_insert(0) += 1;
        }
        counts
    }
}

impl eframe::App for FileTypeCounter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.add_dropped_files(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Type Counter");
            ui.label("Drag files into this window to add them.");
            ui.add_space(8.0);

            ui.columns(2, |columns| {
                columns[0].heading("Files");
                egui::ScrollArea::vertical()
                    .id_salt("files")
                    .max_height(390.0)
                    .show(&mut columns[0], |ui| {
                        if self.files.is_empty() {
                            ui.weak("No files dropped yet.");
                        }
                        for file in &self.files {
                            ui.label(file.display().to_string());
                        }
                    });

                columns[1].heading("File types");
                egui::ScrollArea::vertical()
                    .id_salt("types")
                    .max_height(390.0)
                    .show(&mut columns[1], |ui| {
                        egui::Grid::new("file_type_counts").striped(true).show(ui, |ui| {
                            ui.strong("Type");
                            ui.strong("Count");
                            ui.end_row();
                            for (file_type, count) in self.counts() {
                                ui.label(file_type);
                                ui.label(count.to_string());
                                ui.end_row();
                            }
                        });
                    });
            });

            ui.add_space(8.0);
            ui.label(format!("{} file(s) loaded", self.files.len()));
        });
    }
}
