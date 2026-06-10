#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod flame;

fn main() {
    let effect = flame::Flame::new();
    library::screensaver_runner::run_main(effect, "flame");
}
