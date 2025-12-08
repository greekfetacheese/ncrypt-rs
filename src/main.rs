#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod gui;

use eframe::{
   egui,
   egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew},
   wgpu::{self, MemoryHints, Trace},
};
use gui::app::NCryptApp;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
   let wgpu_setup = WgpuSetup::CreateNew(WgpuSetupCreateNew {
      device_descriptor: Arc::new(|_adapter| wgpu::DeviceDescriptor {
         memory_hints: MemoryHints::MemoryUsage,
         trace: Trace::Off,
         ..Default::default()
      }),
      ..Default::default()
   });

   let wgpu_config = WgpuConfiguration {
      wgpu_setup,
      ..Default::default()
   };

   let options = eframe::NativeOptions {
      wgpu_options: wgpu_config,
      viewport: egui::ViewportBuilder::default()
         .with_drag_and_drop(true)
         .with_decorations(true)
         .with_inner_size([960.0, 550.0])
         .with_min_inner_size([960.0, 550.0])
         .with_transparent(true),
      ..Default::default()
   };

   eframe::run_native(
      "nCrypt",
      options,
      Box::new(|cc| Ok(Box::new(NCryptApp::new(cc)))),
   )
}
