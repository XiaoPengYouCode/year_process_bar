use std::path::Path;
use std::sync::Arc;

pub fn config_fonts(ctx: &egui::Context) -> Result<(), Box<dyn std::error::Error>> {
    let mut fonts = egui::FontDefinitions::default();

    let font_path = Path::new("src/assets/fonts/NotoSansCJKsc-Regular.otf");
    let font_data = std::fs::read(font_path)?;
    let custom_font = egui::FontData::from_owned(font_data).tweak(egui::FontTweak {
        scale: 1.0,
        y_offset_factor: 0.0,
        y_offset: 0.0,
        ..Default::default()
    });

    fonts
        .font_data
        .insert("Noto".to_string(), Arc::new(custom_font));
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Noto".to_string());

    ctx.set_fonts(fonts);
    Ok(())
}
