#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // makes the terminal not show on windows

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Main_Window::new()?;
    main_window.run()
}
