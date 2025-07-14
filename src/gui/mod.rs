use eframe::egui::{Align2, Button, Color32, RichText, Slider, Spinner, Ui, Window, vec2};
use egui_theme::{Theme, ThemeKind};
use lazy_static::lazy_static;
use ncrypt_me::Argon2;
use std::sync::{Arc, RwLock};

lazy_static! {
   pub static ref SHARED_GUI: Arc<RwLock<GUI>> = Arc::new(RwLock::new(GUI::new(Theme::new(ThemeKind::Mocha))));
}

const M_COST_TIP: &str =
    "How much memory the Argon2 algorithm uses. Higher values are more secure but way slower, make sure the memory cost does not exceed your computer RAM.
    You probably want to just increase the Memory cost to a sensible value 256mb - 1024mb as this is the most important parameter for security.";

const T_COST_TIP: &str =
   "The number of iterations the Argon2 algorithm will run. Higher values are more secure but slower.";

const P_COST_TIP: &str = "You should probably leave this to 1.";

use file_encryption::FileEncryptionUi;
use text_hashing::TextHashingUi;

pub mod file_encryption;
pub mod text_hashing;

pub struct MessageWindow {
   pub open: bool,
   pub message: String,
   pub loading: bool,
   pub size: (f32, f32),
}

impl MessageWindow {
   pub fn open_with_msg(&mut self, msg: impl Into<String>) {
      self.open = true;
      self.loading = false;
      self.message = msg.into();
   }

   pub fn open_with_loading(&mut self, msg: impl Into<String>) {
      self.open = true;
      self.message = msg.into();
      self.loading = true;
   }

   pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
      if !self.open {
         return;
      }

      Window::new("msg_window")
         .title_bar(false)
         .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
         .show(ui.ctx(), |ui| {
            ui.set_width(self.size.0);
            ui.set_height(self.size.1);

            ui.vertical_centered(|ui| {
               ui.add_space(20.0);
               ui.spacing_mut().item_spacing.y = 20.0;
               ui.spacing_mut().button_padding = vec2(10.0, 10.0);

               if self.loading {
                  ui.add(Spinner::new().size(20.0).color(Color32::WHITE));
                  ui.add_space(10.0);
                  ui.label(RichText::new(self.message.clone()).size(theme.text_sizes.normal));
               } else {
                  ui.label(RichText::new(self.message.clone()).size(theme.text_sizes.normal));
                  if ui
                     .add(Button::new(
                        RichText::new("Ok").size(theme.text_sizes.normal),
                     ))
                     .clicked()
                  {
                     self.open = false;
                  }
               }
            });
         });
   }
}

pub struct GUI {
   pub theme: Theme,
   pub file_encryption: FileEncryptionUi,
   pub text_hashing: TextHashingUi,
   pub argon2: Argon2,
   pub msg_window: MessageWindow,
}

impl GUI {
   pub fn new(theme: Theme) -> Self {
      let msg_window = MessageWindow {
         open: false,
         message: String::new(),
         loading: false,
         size: (250.0, 150.0),
      };

      let argon2 = Argon2::balanced();

      Self {
         theme,
         file_encryption: FileEncryptionUi::new(),
         text_hashing: TextHashingUi::new(),
         argon2,
         msg_window,
      }
   }

   pub fn show_left_panel(&mut self, ui: &mut Ui) {
      ui.vertical(|ui| {
         ui.spacing_mut().item_spacing.y = 20.0;
         ui.spacing_mut().button_padding = vec2(10.0, 10.0);

         let text = RichText::new("File Encryption").size(self.theme.text_sizes.normal);
         let text2 = RichText::new("Text Hashing").size(self.theme.text_sizes.normal);

         ui.horizontal(|ui| {
            if ui.add(Button::new(text)).clicked() {
               self.file_encryption.open = true;
               self.text_hashing.open = false;
            }
         });

         ui.horizontal(|ui| {
            if ui.add(Button::new(text2)).clicked() {
               self.file_encryption.open = false;
               self.text_hashing.open = true;
            }
         });
      });
   }

   pub fn show_right_panel(&mut self, ui: &mut Ui) {
      // Argon Params

      ui.vertical_centered(|ui| {
         ui.spacing_mut().item_spacing.y = 20.0;
         ui.spacing_mut().button_padding = vec2(10.0, 10.0);

         ui.label(RichText::new("Argon2 Parameters").size(self.theme.text_sizes.normal));

         ui.label(RichText::new("Memory cost (MB):").size(self.theme.text_sizes.normal))
            .on_hover_text(M_COST_TIP);

         ui.add(
            Slider::new(&mut self.argon2.m_cost, 64_000..=10000000) // 64MB - 10GB
               .custom_formatter(|v, _ctx| format!("{:.0} MB", v / 1000.0)),
         );

         ui.label(RichText::new("Iterations:").size(self.theme.text_sizes.normal))
            .on_hover_text(T_COST_TIP);

         ui.add(Slider::new(&mut self.argon2.t_cost, 5..=1024));

         ui.label(RichText::new("Parallelism:").size(self.theme.text_sizes.normal))
            .on_hover_text(P_COST_TIP);

         ui.add(Slider::new(&mut self.argon2.p_cost, 1..=256));
      });
   }

   pub fn show_central_panel(&mut self, ui: &mut Ui) {
      self.msg_window.show(&self.theme, ui);
      self
         .file_encryption
         .show(&self.theme, self.argon2.clone(), ui);
      self.text_hashing.show(&self.theme, ui);
   }
}
