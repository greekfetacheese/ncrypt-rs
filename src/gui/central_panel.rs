use eframe::egui::{ vec2, Align2, Button, Frame, RichText, Ui, Vec2b, Window };
use std::sync::{ Arc, RwLock };
use super::{ui::*, GUI};



pub fn show(ui: &mut Ui, gui: &mut GUI) {

   ui.horizontal(|ui| {
    // manually center ui
    ui.add_space(90.0);

    ui.vertical_centered(|ui| {
    gui.encryption_ui.show(&gui.theme, ui);
    gui.text_hashing_ui.show(&gui.theme, ui);
    });

});

    window_msg(ui, gui.pop_msg.clone());
}



/// Show a popup window with a message
pub fn window_msg(ui: &mut Ui, pop_msg: Arc<RwLock<WindowMsg>>) {
    let msg;
    let title;
    {
        let pop_msg = pop_msg.read().unwrap();
        if !pop_msg.open {
            return;
        }
        msg = pop_msg.message.clone();
        title = pop_msg.title.clone();
    }

    Window::new(RichText::new(title).size(16.0))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(Vec2b::new(false, false))
        .frame(Frame::window(&ui.style().clone()))
        .show(ui.ctx(), |ui| {
            ui.set_min_size(vec2(300.0, 150.0));

            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 15.0;

                ui.label(RichText::new(msg).size(14.0));

                let button = Button::new(RichText::new("Ok").size(15.0)).min_size((40.0, 25.0).into());
                
                if ui.add(button).clicked() {
                    let mut pop_msg = pop_msg.write().unwrap();
                    pop_msg.open = false;
                }
            });
        });
}