use crate::font::config_fonts;
use chrono::Datelike;

#[derive(Default)]
pub struct App {
    pub date: String,
    pub process: u32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        config_fonts(&cc.egui_ctx).expect("Failed to load custom font");
        let today = chrono::Local::now();
        let total_days = if today.with_ordinal(366).is_some() {
            366
        } else {
            365
        };
        let days_passed = today.ordinal();

        let process = (days_passed as f32 / total_days as f32 * 100.0) as u32;
        let date = today.format("%Y-%m-%d").to_string();

        Self { date, process }
    }
}
