use super::*;
use eframe::egui::{DroppedFile, Grid, Label, RichText, Ui};
use ncrypt_me::{Argon2, Credentials, decrypt_data, encrypt_data, secure_types::SecureBytes};
use zeus_theme::Theme;
use zeus_ui_components::SecureInputField;
use zeus_widgets::Button;

const FILE_EXTENSION: &str = ".ncrypt";

/// File Encryption/Decryption Ui
pub struct FileEncryptionUi {
   pub open: bool,
   pub credentials: Credentials,
   pub username_field: SecureInputField,
   pub password_field: SecureInputField,
   pub confirm_password_field: SecureInputField,
   pub file_path: String,
   pub dropped_file: Option<DroppedFile>,
}

impl FileEncryptionUi {
   pub fn new() -> Self {
      Self {
         open: true,
         credentials: Credentials::new_with_capacity(1024).unwrap(),
         username_field: SecureInputField::new("Username", false, true),
         password_field: SecureInputField::new("Password", true, true),
         confirm_password_field: SecureInputField::new("Confirm Password", true, true),
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
         ui.spacing_mut().button_padding = vec2(5.0, 5.0);

         let text =
            RichText::new("You can drag and drop your file here or select a file").size(theme.text_sizes.normal);
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
            self.file_path = dropped_file
               .path
               .as_ref()
               .unwrap()
               .to_string_lossy()
               .to_string();
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
         ui.label(RichText::new("Enter Your Credentials").size(theme.text_sizes.normal));

         let user_output = self.username_field.show(theme, ui);
         let passwd_output = self.password_field.show(theme, ui);
         let confirm_passwd_output = self.confirm_password_field.show(theme, ui);

         match user_output {
            Some(output) => {
               if output.response.changed() {
                  self.credentials.username = self.username_field.text();
               }
            }
            None => {}
         }

         match passwd_output {
            Some(output) => {
               if output.response.changed() {
                  self.credentials.password = self.password_field.text();
               }
            }
            None => {}
         }

         match confirm_passwd_output {
            Some(output) => {
               if output.response.changed() {
                  self.credentials.confirm_password = self.confirm_password_field.text();
               }
            }
            None => {}
         };

         ui.add_sized(vec2(150.0, 30.0), |ui: &mut Ui| {
            ui.spacing_mut().button_padding = vec2(10.0, 10.0);

            let res = Grid::new("encrypt_decrypt_grid")
               .spacing(vec2(10.0, 0.0))
               .show(ui, |ui| {
                  self.encrypt(theme, argon2, ui);
                  self.decrypt(theme, ui);
               });
            res.response
         });
      });
   }

   fn encrypt(&mut self, theme: &Theme, argon2: Argon2, ui: &mut Ui) {
      let text = RichText::new("Encrypt").size(theme.text_sizes.normal);
      let visuals = theme.button_visuals();
      let button = Button::new(text).visuals(visuals);

      if ui.add(button).clicked() {
         let file_path = self.file_path.clone();
         let credentials = self.credentials.clone();
         std::thread::spawn(move || {
            let data = match std::fs::read(&file_path) {
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

            let secure_data = match SecureBytes::from_vec(data) {
               Ok(data) => data,
               Err(e) => {
                  SHARED_GUI.write(|gui| {
                     gui.msg_window
                        .open_with_msg(format!("Error creating secure data: {}", e));
                  });
                  return;
               }
            };

            let encrypted_data = match encrypt_data(argon2, secure_data, credentials) {
               Ok(data) => data,
               Err(e) => {
                  SHARED_GUI.write(|gui| {
                     gui.msg_window
                        .open_with_msg(format!("Error encrypting file: {}", e));
                  });
                  return;
               }
            };

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
   }

   fn decrypt(&mut self, theme: &Theme, ui: &mut Ui) {
      let text = RichText::new("Decrypt").size(theme.text_sizes.normal);
      let visuals = theme.button_visuals();
      let button = Button::new(text).visuals(visuals);

      if ui.add(button).clicked() {
         let file_path = self.file_path.clone();
         let credentials = self.credentials.clone();

         std::thread::spawn(move || {
            let data = match std::fs::read(&file_path) {
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

            let decrypted_data = match decrypt_data(data, credentials) {
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

            decrypted_data.unlock_slice(|data| match std::fs::write(&new_file_path, data) {
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
            });
         });
      }
   }
}
