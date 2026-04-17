use eframe::egui::{FontId, Margin, RichText, Ui, vec2};
use ncrypt_me::secure_types::SecureString;
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use zeus_theme::Theme;
use zeus_widgets::SecureTextEdit;
use zeus_widgets::{Button, ComboBox, Label};

#[derive(Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
   Sha3_224,
   Sha3_256,
   Sha3_384,
   Sha3_512,
}

impl HashAlgorithm {
   pub fn to_string(&self) -> String {
      (match self {
         HashAlgorithm::Sha3_224 => "SHA3-224",
         HashAlgorithm::Sha3_256 => "SHA3-256",
         HashAlgorithm::Sha3_384 => "SHA3-384",
         HashAlgorithm::Sha3_512 => "SHA3-512",
      })
      .to_string()
   }

   pub fn to_vec(&self) -> Vec<HashAlgorithm> {
      vec![
         HashAlgorithm::Sha3_224,
         HashAlgorithm::Sha3_256,
         HashAlgorithm::Sha3_384,
         HashAlgorithm::Sha3_512,
      ]
   }
}

pub struct TextHashingUi {
   pub open: bool,
   pub algorithm: HashAlgorithm,
   pub input_text: SecureString,
   pub output_hash: SecureString,
}

impl TextHashingUi {
   pub fn new() -> Self {
      Self {
         open: false,
         algorithm: HashAlgorithm::Sha3_224,
         input_text: SecureString::new_with_capacity(1024).unwrap(),
         output_hash: SecureString::new_with_capacity(1024).unwrap(),
      }
   }

   pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
      if !self.open {
         return;
      }

      ui.vertical_centered(|ui| {
         ui.set_width(ui.available_width() * 0.8);
         ui.set_height(ui.available_height() * 0.8);
         ui.spacing_mut().item_spacing.y = 25.0;
         ui.spacing_mut().button_padding = vec2(10.0, 10.0);

         ui.add_space(10.0);
         self.select_algorithm(theme, ui);

         ui.label(RichText::new("Input Text").size(theme.text_sizes.large));

         let mut should_calculate = false;

         self.input_text.unlock_mut(|input_text| {
            let text_edit = SecureTextEdit::multiline(input_text)
               .desired_width(300.0)
               .desired_rows(5)
               .margin(Margin::same(10))
               .font(FontId::proportional(theme.text_sizes.normal));
            let output = text_edit.show(ui);
            if output.response.changed() {
               should_calculate = true;
            }
         });

         if should_calculate {
            self.calculate_hash();
         }

         ui.label(RichText::new("Hash Output").size(theme.text_sizes.large));

         self.output_hash.unlock_mut(|output_hash| {
            let text_edit = SecureTextEdit::multiline(output_hash)
               .desired_width(300.0)
               .desired_rows(5)
               .margin(Margin::same(10))
               .font(FontId::proportional(theme.text_sizes.normal));
            text_edit.show(ui);
         });

         let visuals = theme.button_visuals();
         let text = RichText::new("Copy").size(theme.text_sizes.normal);
         let button = Button::new(text).visuals(visuals);

         if ui.add(button).clicked() {
            self.output_hash.unlock_str(|text| {
               ui.ctx().copy_text(text.to_owned());
            })
         }

         self.input_text.unlock_str(|input_text| {
            if input_text.is_empty() {
               self.output_hash.erase();
            }
         });
      });
   }

   pub fn calculate_hash(&mut self) {
      self.input_text.unlock_str(|input_text| {
         if input_text.is_empty() {
            return;
         }

         if self.algorithm == HashAlgorithm::Sha3_224 {
            let mut hasher = Sha3_224::new();
            hasher.update(input_text.as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
         } else if self.algorithm == HashAlgorithm::Sha3_256 {
            let mut hasher = Sha3_256::new();
            hasher.update(input_text.as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
         } else if self.algorithm == HashAlgorithm::Sha3_384 {
            let mut hasher = Sha3_384::new();
            hasher.update(input_text.as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
         } else {
            let mut hasher = Sha3_512::new();
            hasher.update(input_text.as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
         }
      });
   }

   fn select_algorithm(&mut self, theme: &Theme, ui: &mut Ui) {
      let label_text = RichText::new(self.algorithm.to_string()).size(theme.text_sizes.normal);
      let label = Label::new(label_text, None);
      let visuals = theme.combo_box_visuals();

      ComboBox::new("select_algo", label)
         .visuals(visuals)
         .width(150.0)
         .show_ui(ui, |ui| {
            ui.spacing_mut().button_padding = vec2(5.0, 5.0);

            let mut algorithms = self.algorithm.to_vec();

            for selected_algorithm in algorithms.iter_mut() {
               let value = ui.selectable_value(
                  &mut self.algorithm,
                  selected_algorithm.clone(),
                  RichText::new(selected_algorithm.to_string()).size(theme.text_sizes.normal),
               );

               if value.clicked() {
                  self.algorithm = selected_algorithm.clone();
                  self.calculate_hash();
               }
            }
         });
   }
}
