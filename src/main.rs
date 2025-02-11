use egui::ViewportBuilder;

mod app;
mod font;
mod ui;

use crate::app::App;

fn main() -> Result<(), eframe::Error> {
    let native_option = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size((400.0, 150.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Year Process Bar",
        native_option,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;
    Ok(())
}
