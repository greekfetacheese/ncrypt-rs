use crate::gui::SHARED_GUI;
use eframe::{
   CreationContext,
   egui::{CentralPanel, Context, Frame, Margin, Rgba, SidePanel, Visuals},
};
use egui_theme::{Theme, ThemeKind};
use window::window_frame;

pub mod window;

/// The main application struct
pub struct NCryptApp {
   pub on_startup: bool,
}

impl NCryptApp {
   pub fn new(cc: &CreationContext) -> Self {
      let theme = Theme::new(ThemeKind::Mocha);
      let app = Self { on_startup: true };

      cc.egui_ctx.set_style(theme.style);
      app
   }
}

impl eframe::App for NCryptApp {
   fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
      Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
   }

   fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
      let mut gui = SHARED_GUI.write().unwrap();

      let bg_color = gui.theme.colors.bg_color;
      let frame = Frame::new().fill(bg_color);

      let left_frame = frame.inner_margin(Margin {
         left: 0,
         right: 0,
         top: 50,
         bottom: 0,
      });

      let right_frame = frame.inner_margin(Margin {
         left: 10,
         right: 0,
         top: 50,
         bottom: 0,
      });

      let central_frame = frame.inner_margin(Margin {
         left: 50,
         right: 0,
         top: 20,
         bottom: 0,
      });

      window_frame(ctx, frame, "nCrypt 1.3.0", |ui| {
         if self.on_startup {
            ctx.set_style(gui.theme.style.clone());
            self.on_startup = false;
         }

         // UI that belongs to the left panel
         SidePanel::left("left_panel")
            .max_width(130.0)
            .resizable(false)
            .frame(left_frame)
            .show_inside(ui, |ui| {
                gui.show_left_panel(ui);
            });

         // UI that belongs to the right panel
         SidePanel::right("right_panel")
         .max_width(200.0)
         .resizable(false)
         .frame(right_frame)
         .show_inside(ui, |ui| {
             gui.show_right_panel(ui);
         });

         // UI that belongs to the central panel
         CentralPanel::default()
            .frame(central_frame)
            .show_inside(ui, |ui| {
                gui.show_central_panel(ui);
            });
      });
   }

   fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    std::thread::spawn(|| {
        let mut gui = SHARED_GUI.write().unwrap();
        gui.text_hashing.input_text.erase();
        gui.text_hashing.output_hash.erase();
        gui.file_encryption.credentials.erase();
    });
   }
}
