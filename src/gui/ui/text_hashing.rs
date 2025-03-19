use eframe::egui::{ Ui, ComboBox, TextEdit, RichText, Color32, FontSelection, FontId };
use egui_theme::Theme;
use sha3::{ Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512 };
use ncrypt_me::secure_types::SecureString;

#[derive(Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

impl HashAlgorithm {
    pub fn to_string(&self) -> String {
        (
            match self {
                HashAlgorithm::Sha3_224 => "SHA3-224",
                HashAlgorithm::Sha3_256 => "SHA3-256",
                HashAlgorithm::Sha3_384 => "SHA3-384",
                HashAlgorithm::Sha3_512 => "SHA3-512",
            }
        ).to_string()
    }

    pub fn to_vec(&self) -> Vec<HashAlgorithm> {
        vec![
            HashAlgorithm::Sha3_224,
            HashAlgorithm::Sha3_256,
            HashAlgorithm::Sha3_384,
            HashAlgorithm::Sha3_512
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
            input_text: SecureString::from(""),
            output_hash: SecureString::from(""),
        }
    }

    pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
        if !self.open {
            return;
        }

        let font = FontSelection::FontId(FontId::monospace(13.0));
        let font_2 = FontSelection::FontId(FontId::monospace(13.0));

        ui.spacing_mut().item_spacing.y = 10.0;
        self.select_algorithm(ui);

        egui_theme::utils::border_on_idle(ui, 1.0, theme.colors.border_color_idle);
        egui_theme::utils::border_on_hover(ui, 1.0, theme.colors.border_color_hover);
        egui_theme::utils::border_on_click(ui, 1.0, theme.colors.border_color_click);

        ui.label(RichText::new("Input Text").size(16.0));

        let mut should_calculate = false;
        self.input_text.string_mut(|input_text| {
            let res = ui.add(
                TextEdit::multiline(input_text)
                    .desired_width(300.0)
                    .desired_rows(10)
                    .text_color(Color32::WHITE)
                    .font(font)
            );
            if res.changed() {
                should_calculate = true;
            }
        });

        if should_calculate {
            self.calculate_hash();
        }

        ui.label(RichText::new("Hash Output").size(16.0));

        self.output_hash.string_mut(|output_hash| {
            ui.add(
                TextEdit::multiline(output_hash)
                    .desired_width(300.0)
                    .desired_rows(10)
                    .text_color(Color32::WHITE)
                    .font(font_2)
            );
        });

        if self.input_text.borrow().is_empty() {
            self.output_hash.erase();
        }
    }

    pub fn calculate_hash(&mut self) {
        if self.input_text.borrow().is_empty() {
            return;
        }
        if self.algorithm == HashAlgorithm::Sha3_224 {
            let mut hasher = Sha3_224::new();
            hasher.update(self.input_text.borrow().as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
        } else if self.algorithm == HashAlgorithm::Sha3_256 {
            let mut hasher = Sha3_256::new();
            hasher.update(self.input_text.borrow().as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
        } else if self.algorithm == HashAlgorithm::Sha3_384 {
            let mut hasher = Sha3_384::new();
            hasher.update(self.input_text.borrow().as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
        } else {
            let mut hasher = Sha3_512::new();
            hasher.update(self.input_text.borrow().as_bytes());
            let result = hasher.finalize();
            self.output_hash = format!("{:x}", result).into();
        }
    }

    fn select_algorithm(&mut self, ui: &mut Ui) {
        ComboBox::from_label("")
            .selected_text(self.algorithm.to_string())
            .show_ui(ui, |ui| {
                let mut algorithms = self.algorithm.to_vec();

                for selected_algorithm in algorithms.iter_mut() {
                    let value = ui.selectable_value(
                        &mut self.algorithm,
                        selected_algorithm.clone(),
                        selected_algorithm.to_string()
                    );

                    if value.clicked() {
                        self.algorithm = selected_algorithm.clone();
                        self.calculate_hash();
                    }
                }
            });
    }
}
