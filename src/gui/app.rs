use crate::gui::{GUI, SHARED_GUI};
use eframe::{
   CreationContext,
   egui::{CentralPanel, Context, Frame, Panel, Rgba, Ui, Visuals},
};
use zeus_theme::{Theme, ThemeKind};

/// The main application struct
pub struct NCryptApp {
   pub style_has_been_set: bool,
}

impl NCryptApp {
   pub fn new(cc: &CreationContext) -> Self {
      let theme = Theme::new(ThemeKind::Dark);

      cc.egui_ctx.set_global_style(theme.style.clone());

      let app = Self {
         style_has_been_set: false,
      };

      app
   }

   fn on_shutdown(&mut self, ctx: &Context, gui: &mut GUI) {
      if ctx.input(|i| i.viewport().close_requested()) {
         gui.file_encryption.credentials.erase();
         gui.text_hashing.input_text.erase();
         gui.text_hashing.output_hash.erase();
      }
   }
}

impl eframe::App for NCryptApp {
   fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
      Rgba::TRANSPARENT.to_array()
   }

   fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
      SHARED_GUI.write(|gui| {
         self.on_shutdown(ui.ctx(), gui);

         // This is needed for Windows
         if !self.style_has_been_set {
            let style = gui.theme.style.clone();
            ui.set_global_style(style);
            self.style_has_been_set = true;
         }

         let theme = &gui.theme;
         let bg_color = theme.colors.bg;
         let panel_frame = Frame::new().fill(bg_color);
         let top_frame = Frame::new().inner_margin(5).fill(bg_color);

         Panel::top("top_panel")
            .min_size(30.0)
            .max_size(30.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(top_frame)
            .show_inside(ui, |_ui| {});

         // UI that belongs to the left panel
         Panel::left("left_panel")
            .max_size(140.0)
            .resizable(false)
            .frame(top_frame)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
               gui.show_left_panel(ui);
            });

         // UI that belongs to the right panel
         Panel::right("right_panel")
            .max_size(200.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(panel_frame)
            .show_inside(ui, |ui| {
               gui.show_right_panel(ui);
            });

         // UI that belongs to the central panel
         CentralPanel::default().frame(panel_frame).show_inside(ui, |ui| {
            gui.show_central_panel(ui);
         });
      });
   }
}
