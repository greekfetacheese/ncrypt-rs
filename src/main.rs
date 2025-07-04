#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;
pub mod gui;

use app::NCryptApp;
use eframe::{
   egui,
   egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew},
   wgpu::{self, Backends, InstanceDescriptor, MemoryHints, PowerPreference, PresentMode},
};
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
   let wgpu_setup = WgpuSetup::CreateNew(WgpuSetupCreateNew {
      instance_descriptor: InstanceDescriptor {
         backends: Backends::VULKAN | Backends::DX12 | Backends::GL,
         ..Default::default()
      },
      power_preference: PowerPreference::HighPerformance,
      device_descriptor: Arc::new(|adapter| {
         let base_limits = if adapter.get_info().backend == wgpu::Backend::Gl {
            wgpu::Limits::downlevel_webgl2_defaults()
         } else {
            wgpu::Limits::default()
         };

         wgpu::DeviceDescriptor {
            label: Some("egui wgpu device"),
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits {
               max_texture_dimension_2d: 8192,
               ..base_limits
            },
            memory_hints: MemoryHints::MemoryUsage,
         }
      }),
      ..Default::default()
   });

   let wgpu_config = WgpuConfiguration {
      present_mode: PresentMode::Fifo,
      desired_maximum_frame_latency: None,
      wgpu_setup,
      ..Default::default()
   };

   let options = eframe::NativeOptions {
      wgpu_options: wgpu_config,
      viewport: egui::ViewportBuilder::default()
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
