#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lapce_ui::app;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub fn main() {
    app::launch();
}
