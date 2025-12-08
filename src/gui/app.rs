use crate::gui::{GUI, SHARED_GUI};
use eframe::{
   CreationContext,
   egui::{CentralPanel, Context, Frame, Rgba, SidePanel, TopBottomPanel, Visuals},
};
use zeus_theme::{Theme, ThemeKind};

/// The main application struct
pub struct NCryptApp {
   pub style_has_been_set: bool,
}

impl NCryptApp {
   pub fn new(cc: &CreationContext) -> Self {
      let theme = Theme::new(ThemeKind::Dark);

      cc.egui_ctx.set_style(theme.style.clone());

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

   fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
      SHARED_GUI.write(|gui| {
         self.on_shutdown(ctx, gui);

         // This is needed for Windows
         if !self.style_has_been_set {
            let style = gui.theme.style.clone();
            ctx.set_style(style);
            self.style_has_been_set = true;
         }

         let theme = &gui.theme;
         let bg_color = theme.colors.bg;
         let panel_frame = Frame::new().fill(bg_color);
         let top_frame = Frame::new().inner_margin(5).fill(bg_color);

         TopBottomPanel::top("top_panel")
            .min_height(30.0)
            .max_height(30.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(top_frame)
            .show(ctx, |_ui| {});

         // UI that belongs to the left panel
         SidePanel::left("left_panel")
            .max_width(140.0)
            .resizable(false)
            .frame(top_frame)
            .show(ctx, |ui| {
               gui.show_left_panel(ui);
            });

         // UI that belongs to the right panel
         SidePanel::right("right_panel")
            .max_width(200.0)
            .resizable(false)
            .frame(panel_frame)
            .show(ctx, |ui| {
               gui.show_right_panel(ui);
            });

         // UI that belongs to the central panel
         CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            gui.show_central_panel(ui);
         });
      });
   }
}
