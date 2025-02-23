use eframe::egui::{ Ui, TextEdit, RichText, Button, Slider };
use num_format::{ Locale, ToFormattedString };
use std::sync::{ Arc, RwLock };
use ncrypt_me::{ Argon2Params, Credentials, decrypt_data, encrypt_data };
use egui_theme::Theme;
use super::*;

const FILE_EXTENSION: &str = ".ncrypt";

/// File Encryption/Decryption Ui
pub struct FileEncryptionUi {
    pub open: bool,

    pub credentials: Credentials,

    pub file_path: String,

    pub argon_params: Argon2Params,

    pub pop_msg: Arc<RwLock<WindowMsg>>,
}

impl FileEncryptionUi {
    pub fn new(pop_msg: Arc<RwLock<WindowMsg>>) -> Self {
        Self {
            open: true,
            credentials: Credentials::default(),
            file_path: String::new(),
            argon_params: Argon2Params::fast(),
            pop_msg,
        }
    }

    pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
        if !self.open {
            return;
        }

        self.open_file_button(ui);
        self.credentials_input(theme, ui);

        ui.horizontal(|ui| {
            ui.add_space(100.0);
            self.encrypt(ui);

            ui.add_space(15.0);

            self.decrypt(ui);
        });
    }

    fn encrypt(&mut self, ui: &mut Ui) {
        let text = RichText::new("Encrypt").size(16.0);
        let button = Button::new(text).min_size((100.0, 30.0).into());
        let ctx = ui.ctx().clone();

        if ui.add(button).clicked() {
            {
                let mut pop_msg = self.pop_msg.write().unwrap();
                pop_msg.open = true;
                pop_msg.message = "Encrypting...".to_string();
            }

            let argon_params = self.argon_params.clone();
            let file_path = self.file_path.clone();
            let credentials = self.credentials.clone();
            let pop_msg = self.pop_msg.clone();

            std::thread::spawn(move || {
                let data = match std::fs::read(file_path.clone()) {
                    Ok(data) => data,
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to read file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                        return;
                    }
                };

                let encrypted_data = match encrypt_data(argon_params.clone(), data, credentials) {
                    Ok(data) => data,
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to encrypt file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                        return;
                    }
                };

                let new_file_path = format!("{}{}", file_path, FILE_EXTENSION);

                match std::fs::write(&new_file_path, &encrypted_data) {
                    Ok(_) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Success".to_string();
                            pop_msg.message =
                                format!("File encrypted successfully to: {}", new_file_path);
                        }
                        ctx.request_repaint();
                    }
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to save the encrypted file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                    }
                }
            });
        }
    }

    fn decrypt(&mut self, ui: &mut Ui) {
        let text = RichText::new("Decrypt").size(16.0);
        let button = Button::new(text).min_size((100.0, 30.0).into());
        let ctx = ui.ctx().clone();

        if ui.add(button).clicked() {
            {
                let mut pop_msg = self.pop_msg.write().unwrap();
                pop_msg.open = true;
                pop_msg.message = "Decrypting...".to_string();
            }

            let file_path = self.file_path.clone();
            let credentials = self.credentials.clone();
            let pop_msg = self.pop_msg.clone();

            std::thread::spawn(move || {
                let data = match std::fs::read(file_path.clone()) {
                    Ok(data) => data,
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to read file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                        return;
                    }
                };

                let decrypted_data = match decrypt_data(data, credentials) {
                    Ok(data) => data,
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to decrypt file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                        return;
                    }
                };

                // remove the extension
                let new_file_path = file_path.replace(FILE_EXTENSION, "");

                match std::fs::write(&new_file_path, &decrypted_data) {
                    Ok(_) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Success".to_string();
                            pop_msg.message =
                                format!("File decrypted successfully to: {}", new_file_path);
                        }
                        ctx.request_repaint();
                    }
                    Err(e) => {
                        {
                            let mut pop_msg = pop_msg.write().unwrap();
                            pop_msg.open = true;
                            pop_msg.title = "Failed to save decrypted file".to_string();
                            pop_msg.message = format!("{:?}", e);
                        }
                        ctx.request_repaint();
                    }
                }
            });
        }
    }

    fn open_file_button(&mut self, ui: &mut Ui) {
        let text = RichText::new("Choose a File").size(16.0);
        let button = Button::new(text).min_size((100.0, 30.0).into());

        ui.spacing_mut().item_spacing.y = 15.0;

        if ui.add(button).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.file_path = path.to_str().unwrap().to_string();
            }
        }

        let file_text = RichText::new(format!("File: {}", self.file_path)).size(16.0);
        ui.label(file_text);

        ui.add_space(15.0);
    }

    fn credentials_input(&mut self, theme: &Theme, ui: &mut Ui) {
        ui.scope(|ui| {
    
        ui.spacing_mut().item_spacing.y = 15.0;
        egui_theme::utils::border_on_idle(ui, 1.0, theme.colors.border_color_idle);
        egui_theme::utils::border_on_hover(ui, 1.0, theme.colors.border_color_hover);
        egui_theme::utils::border_on_click(ui, 1.0, theme.colors.border_color_click);

        ui.label(RichText::new("Enter Your Credentials").size(18.0));

        ui.label(RichText::new("Username:").size(16.0));

        // username input
        ui.add(TextEdit::singleline(self.credentials.user_mut()).min_size((200.0, 25.0).into()));

        ui.label(RichText::new("Password:").size(16.0));

        // password input
        ui.add(
            TextEdit::singleline(self.credentials.passwd_mut())
                .min_size((200.0, 25.0).into())
                .password(true)
        );

        ui.label(RichText::new("Confrim Password:").size(16.0));

        // confirm password input
        ui.add(
            TextEdit::singleline(self.credentials.confirm_passwd_mut())
                .min_size((200.0, 25.0).into())
                .password(true)
        );

        ui.add_space(15.0);
    });
    }

    pub fn argon_params_ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 15.0;

            ui.label(RichText::new("Memory Cost (kB)").size(16.0));

            ui.add(
                Slider::new(&mut self.argon_params.m_cost, 2048..=10_000_000)
                    .drag_value_speed(100.0)
                    .custom_formatter(|v, _ctx| {
                        let v_as_int = v.round() as u32;
                        let formatted = v_as_int.to_formatted_string(&Locale::en);
                        format!("{}", formatted)
                    })
            );

            ui.label(RichText::new("Iterations").size(16.0));

            ui.add(
                Slider::new(&mut self.argon_params.t_cost, 1..=5000)
                    .drag_value_speed(100.0)
                    .custom_formatter(|v, _ctx| {
                        let v_as_int = v.round() as u32;
                        let formatted = v_as_int.to_formatted_string(&Locale::en);
                        format!("{}", formatted)
                    })
            );

            ui.label(RichText::new("Parallelism").size(16.0));

            ui.add(Slider::new(&mut self.argon_params.p_cost, 1..=64));
        });
    }
}
