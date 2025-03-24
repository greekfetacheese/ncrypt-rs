#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;
pub mod gui;

use app::NCryptApp;
use eframe::egui::ViewportBuilder;

fn main() -> Result<(), eframe::Error> {
   let options = eframe::NativeOptions {
      viewport: ViewportBuilder::default()
         .with_drag_and_drop(true)
         .with_decorations(false) // Hide the OS-specific "chrome" around the window
         .with_inner_size([960.0, 550.0])
         .with_min_inner_size([960.0, 550.0])
         .with_transparent(true), // To have rounded corners we need transparency
      ..Default::default()
   };
   eframe::run_native(
      "nCrypt",
      options,
      Box::new(|cc| Ok(Box::new(NCryptApp::new(cc)))),
   )
}
