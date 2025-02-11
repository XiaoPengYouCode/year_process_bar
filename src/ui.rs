use egui::{Color32, RichText};

use crate::app::App;

impl App {
    fn render_year_process_bar(&self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add({
                    egui::ProgressBar::new(self.process as f32 / 100.0)
                        .corner_radius(2)
                        .desired_height(30.0)
                        .fill(Color32::from_rgb(80, 180, 80))
                        .text(RichText::new(format!("{}%", self.process)).color(Color32::DARK_GRAY))
                });
            });
        });
    }

    fn render_development_message(&self, ui: &mut egui::Ui) {
        let bottom = egui::TopBottomPanel::bottom("message")
            .show_separator_line(true)
            .resizable(false);
        bottom.show_inside(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Designed by @XiaoPengYouCode | Version 0.1.0");
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut year = self.date.clone();
            let _ = year.split_off(4);
            ui.heading(format!(
                "今天是 {} \n{}年已经过去了：{}%",
                self.date, year, self.process
            ));
            ui.separator();
            self.render_year_process_bar(ui);
            self.render_development_message(ui);
        });
    }
}
