pub mod ui;
pub mod central_panel;
pub mod left_panel;
pub mod right_panel;

use std::sync::{Arc, RwLock};
use ui::{WindowMsg, file_encryption::FileEncryptionUi, text_hashing::TextHashingUi};
use egui_theme::Theme;

pub struct GUI {
    pub theme: Theme,
    pub encryption_ui: FileEncryptionUi,
    pub text_hashing_ui: TextHashingUi,
    pub pop_msg: Arc<RwLock<WindowMsg>>
}

impl GUI {
    pub fn new(theme: Theme) -> Self {
        let pop_msg = Arc::new(RwLock::new(WindowMsg::default()));
        Self {
            theme,
            encryption_ui: FileEncryptionUi::new(pop_msg.clone()),
            text_hashing_ui: TextHashingUi::new(),
            pop_msg
        }
    }
}