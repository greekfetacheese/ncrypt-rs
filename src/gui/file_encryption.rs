use super::*;
use eframe::egui::{Button, DroppedFile, FontId, Grid, Label, Margin, RichText, TextEdit, Ui};
use egui_theme::Theme;
use ncrypt_me::{Argon2Params, Credentials, decrypt_data, encrypt_data};

const FILE_EXTENSION: &str = ".ncrypt";

/// File Encryption/Decryption Ui
pub struct FileEncryptionUi {
   pub open: bool,
   pub credentials: Credentials,
   pub file_path: String,
   pub dropped_file: Option<DroppedFile>,
}

impl FileEncryptionUi {
   pub fn new() -> Self {
      Self {
         open: true,
         credentials: Credentials::default(),
         file_path: String::new(),
         dropped_file: None,
      }
   }

   pub fn show(&mut self, theme: &Theme, argon_params: Argon2Params, ui: &mut Ui) {
      if !self.open {
         return;
      }

      ui.vertical_centered(|ui| {
         ui.set_width(ui.available_width());
         ui.set_height(ui.available_height());
         ui.spacing_mut().item_spacing.y = 15.0;
         ui.spacing_mut().button_padding = vec2(10.0, 10.0);

         let text = RichText::new("You can drag and drop your file here or select a file")
            .size(theme.text_sizes.normal);
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

         if ui
            .add(Button::new(
               RichText::new("Choose a File").size(theme.text_sizes.normal),
            ))
            .clicked()
         {
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
            let file_text = RichText::new(&self.file_path)
               .size(theme.text_sizes.small)
               .strong();
            ui.label(file_text);
         }
         ui.add_space(10.0);

         // Credentials
         ui.label(RichText::new("Enter Your Credentials").size(theme.text_sizes.normal));

         ui.label(RichText::new("Username").size(theme.text_sizes.normal));
         self.credentials.username.secure_mut(|username| {
            let text_edit = TextEdit::singleline(username)
               .margin(Margin::same(10))
               .min_size((200.0, 25.0).into())
               .font(FontId::proportional(theme.text_sizes.normal));
            let mut output = text_edit.show(ui);
            output.state.clear_undoer();
         });

         ui.label(RichText::new("Password").size(theme.text_sizes.normal));
         self.credentials.password.secure_mut(|passwd| {
            let text_edit = TextEdit::singleline(passwd)
               .margin(Margin::same(10))
               .min_size((200.0, 25.0).into())
               .font(FontId::proportional(theme.text_sizes.normal))
               .password(true);
            let mut output = text_edit.show(ui);
            output.state.clear_undoer();
         });

         ui.label(RichText::new("Confirm Password").size(theme.text_sizes.normal));
         self.credentials.confirm_password.secure_mut(|passwd| {
            let text_edit = TextEdit::singleline(passwd)
               .margin(Margin::same(10))
               .min_size((200.0, 25.0).into())
               .font(FontId::proportional(theme.text_sizes.normal))
               .password(true);
            let mut output = text_edit.show(ui);
            output.state.clear_undoer();
         });

         ui.add_sized(vec2(150.0, 30.0), |ui: &mut Ui| {
         let res = Grid::new("encrypt_decrypt_grid")
            .spacing(vec2(10.0, 0.0))
            .show(ui, |ui| {
               self.encrypt(theme, argon_params, ui);
               self.decrypt(theme, ui);
            });
            res.response
         });
      });
   }

   fn encrypt(&mut self, theme: &Theme, argon_params: Argon2Params, ui: &mut Ui) {
      let text = RichText::new("Encrypt").size(theme.text_sizes.normal);

      if ui.add(Button::new(text)).clicked() {
         let file_path = self.file_path.clone();
         let credentials = self.credentials.clone();
         std::thread::spawn(move || {

            let data = match std::fs::read(&file_path) {
               Ok(data) => data,
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error reading file: {}", e));
                  return;
               }
            };

            {
               let mut gui = SHARED_GUI.write().unwrap();
               gui.msg_window.open_with_loading("Encrypting...");
            }

            let encrypted_data = match encrypt_data(argon_params, data, credentials) {
               Ok(data) => data,
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error encrypting file: {}", e));
                  return;
               }
            };

            let new_file_path = format!("{}{}", file_path, FILE_EXTENSION);

            match std::fs::write(&new_file_path, encrypted_data) {
               Ok(_) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("File encrypted successfully to {}", new_file_path));
               }
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error writing file: {}", e));
               }
            }
         });
      }
   }

   fn decrypt(&mut self, theme: &Theme, ui: &mut Ui) {
      let text = RichText::new("Decrypt").size(theme.text_sizes.normal);
      if ui.add(Button::new(text)).clicked() {
         let file_path = self.file_path.clone();
         let credentials = self.credentials.clone();

         std::thread::spawn(move || {

            let data = match std::fs::read(&file_path) {
               Ok(data) => data,
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error reading file: {}", e));
                  return;
               }
            };

            {
               let mut gui = SHARED_GUI.write().unwrap();
               gui.msg_window.open_with_loading("Decrypting...");
            }

            let decrypted_data = match decrypt_data(data, credentials) {
               Ok(data) => data,
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error decrypting file: {}", e));
                  return;
               }
            };

            // remove the extension
            let new_file_path = file_path.replace(FILE_EXTENSION, "");

            match std::fs::write(&new_file_path, decrypted_data.borrow()) {
               Ok(_) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("File decrypted successfully to {}", new_file_path));
               }
               Err(e) => {
                  let mut gui = SHARED_GUI.write().unwrap();
                  gui.msg_window
                     .open_with_msg(format!("Error writing file: {}", e));
               }
            }
         });
      }
   }
}
