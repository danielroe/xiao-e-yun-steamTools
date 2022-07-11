#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use module::invoke_handler;

extern crate macro_tool;

mod module;

fn main() {
  let builder = invoke_handler(tauri::Builder::default());
  
  builder
    .run(tauri::generate_context!("tauri.conf.json"))
    .expect("error while running tauri application");
}