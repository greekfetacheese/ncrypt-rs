#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod gui;

use eframe::{
   egui,
   egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew},
   wgpu::{self, InstanceDescriptor, MemoryHints, Trace},
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
      instance_descriptor: InstanceDescriptor::new_without_display_handle(),
      display_handle: None,
      native_adapter_selector: None,
      power_preference: wgpu::PowerPreference::LowPower,
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
      "nCrypt 2.0.0",
      options,
      Box::new(|cc| {
         egui_extras::install_image_loaders(&cc.egui_ctx);

         let app = NCryptApp::new(cc);

         Ok(Box::new(app))
      }),
   )
}
