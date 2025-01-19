use eframe::egui::Ui;
use super::{GUI, ui::{button, rich_text}};

pub fn show(ui: &mut Ui, gui: &mut GUI) {
    ui.set_max_width(120.0);

    ui.vertical_centered(|ui| {
        ui.spacing_mut().item_spacing.y = 20.0;

        let encryption = button(rich_text("Encryption").size(16.0)).min_size((60.0, 25.0).into());
        if ui.add(encryption).clicked() {
            gui.text_hashing_ui.open = false;
            gui.encryption_ui.open = true;
        }

        let hashing = button(rich_text("Text Hashing").size(16.0)).min_size((60.0, 25.0).into());
        if ui.add(hashing).clicked() {
            gui.text_hashing_ui.open = true;
            gui.encryption_ui.open = false;
        }

});
}