use eframe::egui::{Button, TextEdit, RichText, Sense, WidgetText};

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



pub fn rich_text(text: impl Into<String>) -> RichText {
    RichText::new(text)
        .size(13.0)
}

pub fn button(text: impl Into<WidgetText>) -> Button<'static> {
    Button::new(text)
        .sense(Sense::click())
        .min_size((40.0, 25.0).into())
}

pub fn text_edit(text: &mut String) -> TextEdit<'_> {
    TextEdit::singleline(text)
        .desired_width(200.0)
}