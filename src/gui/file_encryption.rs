use super::*;
use eframe::egui::{Align, DroppedFileHandle, Label, Layout, RichText, Ui};
use ncrypt_me::{Argon2, Credentials, decrypt::decrypt_data_unsecured, encrypt::encrypt_data_ref};
use zeus_theme::Theme;
use zeus_ui_components::CredentialsForm;
use zeus_widgets::{Button, Zeroize};

const FILE_EXTENSION: &str = ".ncrypt";

/// File Encryption/Decryption Ui
pub struct FileEncryptionUi {
   pub open: bool,
   pub credentials_form: CredentialsForm,
   pub file_path: String,
   pub dropped_file: Option<DroppedFileHandle>,
}

impl FileEncryptionUi {
   pub fn new() -> Self {
      let form = CredentialsForm::new()
         .with_open(true)
         .with_confirm_password(true);
      Self {
         open: true,
         credentials_form: form,
         file_path: String::new(),
         dropped_file: None,
      }
   }

   pub fn show(&mut self, theme: &Theme, argon2: Argon2, ui: &mut Ui) {
      if !self.open {
         return;
      }

      ui.vertical_centered(|ui| {
         ui.set_width(ui.available_width());
         ui.set_height(ui.available_height());
         ui.spacing_mut().item_spacing.y = 15.0;
         ui.spacing_mut().button_padding = vec2(10.0, 8.0);

         let text = RichText::new("Drag and drop or select a file").size(theme.text_sizes.normal);
         let label = Label::new(text).wrap();
         ui.scope(|ui| {
            ui.set_max_width(ui.available_width() * 0.5);
            ui.add(label);
         });

         // Collect dropped file
         ui.ctx().input(|i| {
            if let Some(first_file) = i.raw.dropped_files.first() {
               self.dropped_file = Some(first_file.clone());
            }
         });

         let button =
            Button::new(RichText::new("Choose a File").size(theme.text_sizes.normal)).visuals(theme.button_visuals());
         if ui.add(button).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
               self.file_path = path.to_str().unwrap().to_string();
            }
         }

         if let Some(dropped_file) = self.dropped_file.as_ref() {
            self.file_path = dropped_file.path().to_string_lossy().to_string();
            self.dropped_file = None;
         }

         if !self.file_path.is_empty() {
            let mut path = self.file_path.clone();
            if path.len() > 50 {
               path = path.chars().take(50).collect::<String>() + "...";
            }
            let file_text = RichText::new(path).size(theme.text_sizes.small).strong();
            ui.label(file_text);
         }

         ui.add_space(10.0);

         // Credentials
         ui.label(RichText::new("Enter Your Credentials").size(theme.text_sizes.large));

         let form_size = vec2(ui.available_width() * 0.5, 10.0);
         self.credentials_form.set_min_size(form_size);
         self.credentials_form.set_icon_size(vec2(20.0, 20.0));

         ui.scope(|ui| {
            ui.spacing_mut().button_padding = vec2(4.0, 4.0);
            self.credentials_form.show(theme, ui);
         });

         let size = vec2(ui.available_width() * 0.5, 30.0);
         ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 20.0;

            let btn_size = vec2(130.0, 30.0);
            let text = RichText::new("Encrypt").size(theme.text_sizes.normal);
            let visuals = theme.button_visuals();
            let button = Button::new(text).visuals(visuals).min_size(btn_size);

            if ui.add(button).clicked() {
               self.encrypt(argon2);
            }

            let text = RichText::new("Decrypt").size(theme.text_sizes.normal);
            let visuals = theme.button_visuals();
            let button = Button::new(text).visuals(visuals).min_size(btn_size);

            if ui.add(button).clicked() {
               self.decrypt();
            }
         });
      });
   }

   fn encrypt(&mut self, argon2: Argon2) {
      let file_path = self.file_path.clone();
      let username = self.credentials_form.username();
      let password = self.credentials_form.password();
      let credentials = Credentials::new(username, password.clone(), password);

      std::thread::spawn(move || {
         let mut data = match std::fs::read(&file_path) {
            Ok(data) => data,
            Err(e) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error reading file: {}", e));
               });
               return;
            }
         };

         SHARED_GUI.write(|gui| {
            gui.msg_window.open_with_loading("Encrypting...");
         });

         let encrypted_data = match encrypt_data_ref(argon2, &data, credentials) {
            Ok(data) => data,
            Err(e) => {
               data.zeroize();
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error encrypting file: {}", e));
               });
               return;
            }
         };

         data.zeroize();

         let new_file_path = format!("{}{}", file_path, FILE_EXTENSION);

         match std::fs::write(&new_file_path, encrypted_data) {
            Ok(_) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("File encrypted successfully to {}", new_file_path));
               });
            }
            Err(e) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error writing file: {}", e));
               });
            }
         }
      });
   }

   fn decrypt(&mut self) {
      let file_path = self.file_path.clone();
      let username = self.credentials_form.username();
      let password = self.credentials_form.password();
      let credentials = Credentials::new(username, password.clone(), password);

      std::thread::spawn(move || {
         let encrypted_data = match std::fs::read(&file_path) {
            Ok(data) => data,
            Err(e) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error reading file: {}", e));
               });
               return;
            }
         };

         SHARED_GUI.write(|gui| {
            gui.msg_window.open_with_loading("Decrypting...");
         });

         let mut decrypted_data = match decrypt_data_unsecured(encrypted_data, credentials) {
            Ok(data) => data,
            Err(e) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error decrypting file: {}", e));
               });
               return;
            }
         };

         // remove the extension
         let new_file_path = file_path.replace(FILE_EXTENSION, "");

         match std::fs::write(&new_file_path, &decrypted_data) {
            Ok(_) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("File decrypted successfully to {}", new_file_path));
               });
            }
            Err(e) => {
               SHARED_GUI.write(|gui| {
                  gui.msg_window
                     .open_with_msg(format!("Error writing file: {}", e));
               });
            }
         }

         decrypted_data.zeroize();
      });
   }
}
