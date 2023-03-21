use super::calc;
use super::filecalc;
use eframe::egui;
struct App {
    ib_items_string: String,
    ib_items: f64,
    ib_boxes: f64,
    bi_boxes_string: String,
    bi_boxes: f64,
    bi_items: f64,
    name: String,
    input_path: Option<String>,
    output_path: Option<String>,
    error: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ib_items_string: "0".to_owned(),
            ib_items: 0.0,
            ib_boxes: 0.0,
            bi_boxes_string: "0".to_owned(),
            bi_boxes: 0.0,
            bi_items: 0.0,
            name: "".to_string(),
            input_path: None,
            output_path: None,
            error: false,
        }
    }
}

pub fn handler() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Minecalc",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minecalc");
            if self.error {
                ui.label("Error");
            }
            ui.horizontal(|ui| {
                let name_label = ui.label("Item Name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            //Items to Boxes
            ui.horizontal(|ui| {
                let name_label = ui.label("Items: ");
                ui.text_edit_singleline(&mut self.ib_items_string)
                    .labelled_by(name_label.id);
                ui.label(format!(
                    ", {} Items of type {}, are {} Boxes",
                    self.ib_items, self.name, self.ib_boxes
                ));
                if ui.button("Calculate!").clicked() {
                    match self.ib_items_string.trim().parse::<f64>() {
                        Ok(val) => {
                            self.ib_items = val;
                            self.error = false;
                        }
                        Err(_) => {
                            self.error = true;
                        }
                    }
                    self.ib_boxes = calc::boxes_from_items(self.ib_items, &self.name)
                    //match calc::boxes_from_items(items, name)
                }
            });
            //Boxes to Items
            ui.horizontal(|ui| {
                let name_label = ui.label("Boxes: ");
                ui.text_edit_singleline(&mut self.bi_boxes_string)
                    .labelled_by(name_label.id);
                ui.label(format!(
                    ", {} Boxes of type {}, are {} Items",
                    self.bi_boxes, self.name, self.bi_items
                ));
                if ui.button("Calculate!").clicked() {
                    match self.bi_boxes_string.trim().parse::<f64>() {
                        Ok(val) => {
                            self.bi_boxes = val;
                            self.error = false;
                        }
                        Err(_) => {
                            self.error = true;
                        }
                    }
                    self.bi_items = calc::item_from_boxes(self.bi_boxes, &self.name);
                }
            });
            //File Calc
            ui.label("Material List to Boxes List");
            ui.horizontal(|ui| {
                if ui.button("Select Input file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.input_path = Some(path.display().to_string());
                    }
                }

                if ui.button("Calculate and select output file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.output_path = Some(path.display().to_string());
                    }
                    if let Some(output_path) = &self.output_path {
                        if let Some(input_path) = &self.input_path {
                            if let Err(_) = filecalc::parse_file(
                                input_path.to_string(),
                                output_path.to_string(),
                            ) {
                                self.error = true;
                            } else {
                                self.error = false;
                            }
                        }
                    }
                }
            });
            if let Some(path) = &self.input_path {
                ui.label(format!("Input file: {}", path));
            }
        });
    }
}
