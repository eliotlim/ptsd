use std::borrow::Borrow;
use std::thread;

use async_std::task::block_on;
use rfd::{AsyncFileDialog, FileHandle};

use crate::Canvas;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    canvas: Canvas,

    #[serde(skip)]
    pickFileChannel: std::sync::mpsc::Sender<bool>,
    #[serde(skip)]
    dataFileChannel: std::sync::mpsc::Receiver<String>,
}

async fn openFile() -> Option<FileHandle> {
    let file = AsyncFileDialog::new()
        .add_filter("text", &["txt"])
        .set_directory(".")
        .pick_file()
        .await;
    match &file {
        Some(file) => {
            println!("selected file: {:?}", file.file_name());
            let data = file.read().await;
            println!("data: {:?}", data);
        }
        None => {}
    };
    file
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (pickFileSender, pickFileReceiver) = std::sync::mpsc::channel();
        let (dataFileSender, dataFileReceiver) = std::sync::mpsc::channel();

        thread::spawn(move || {
            block_on(async move {
                loop {
                    match pickFileReceiver.recv() {
                        Ok(_) => {
                            let file = AsyncFileDialog::new()
                                .add_filter("text", &["txt"])
                                .set_directory(".")
                                .pick_file()
                                .await;
                            match &file {
                                Some(file) => {
                                    println!("selected file: {:?}", file.file_name());
                                    let data = file.read().await;
                                    println!("data: {:?}", data);
                                    dataFileSender.send(file.file_name().to_string()).expect("TODO: panic message");
                                }
                                None => {}
                            };
                        }
                        Err(_) => {}
                    }
                }
            });
        });

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            canvas: Canvas::new(),
            pickFileChannel: pickFileSender,
            dataFileChannel: dataFileReceiver,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value, canvas, pickFileChannel, dataFileChannel } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {}
                    let openButton = ui.button("Open");
                    if openButton.clicked() {
                        pickFileChannel.send(true).expect("TODO: panic message");
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            canvas.ui(ui);
        });
    }
}
