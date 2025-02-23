pub mod file_encryption;
pub mod text_hashing;

pub struct WindowMsg {
    pub open: bool,
    pub message: String,
    pub title: String,
}

impl Default for WindowMsg {
    fn default() -> Self {
        Self {
            open: false,
            message: String::new(),
            title: String::new()
        }
    }
}