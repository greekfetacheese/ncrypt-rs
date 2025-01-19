use eframe::{
    egui::{
        CentralPanel,
        Context,
        Frame,
        Margin,
        Rgba,
        SidePanel,
        Visuals,
    },
    CreationContext,
};
use encryption::zeroize::Zeroize;
use egui_theme::{Theme, ThemeKind};
use crate::gui::{ central_panel, left_panel, right_panel, GUI };
use window::window_frame;

pub mod window;

/// The main application struct
pub struct NCryptApp {
    pub gui: GUI,
    pub on_startup: bool,
}

impl NCryptApp {
    pub fn new(cc: &CreationContext) -> Self {
        let theme = Theme::new(ThemeKind::Midnight);
        let app = Self {
            gui: GUI::new(theme.clone()),
            on_startup: true,
        };

        cc.egui_ctx.set_style(theme.style);
        app
    }
}

impl eframe::App for NCryptApp {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let bg_color = self.gui.theme.colors.bg_color;
        let frame = Frame::none().fill(bg_color);

        window_frame(ctx, frame, "nCrypt 1.0.0", |ui| {
            if self.on_startup {
                ctx.set_style(self.gui.theme.style.clone());
                self.on_startup = false;
            }

            // UI that belongs to the right panel
            SidePanel::right("right_panel")
                .min_width(50.0)
                .resizable(false)
                .frame(
                    frame.clone().inner_margin(Margin { left: 20.0, right: 0.0, top: 100.0, bottom: 0.0 })
                )
                .show_inside(ui, |ui| {
                    right_panel::show(ui, &mut self.gui);
                });
    

            // UI that belongs to the left panel
            SidePanel::left("left_panel")
                .min_width(50.0)
                .resizable(false)
                .frame(
                    frame.clone().inner_margin(Margin { left: 0.0, right: 0.0, top: 100.0, bottom: 0.0 })
                )
                .show_inside(ui, |ui| {
                    left_panel::show(ui, &mut self.gui);
                });

            // UI that belongs to the central panel
            CentralPanel::default()
                .frame(
                    frame.inner_margin(Margin { left: 0.0, right: 0.0, top: 30.0, bottom: 0.0 })
                )
                .show_inside(ui, |ui| {
                    central_panel::show(ui, &mut self.gui);
                });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.gui.encryption_ui.credentials.destroy();
        self.gui.text_hashing_ui.input_text.zeroize();
        self.gui.text_hashing_ui.output_hash.zeroize();
    }
}