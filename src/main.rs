//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // makes the terminal not show on windows //disabled for now

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Main_Window::new()?;

    main_window.on_install_horde(|| {
        install_horde();
    });

    main_window.run()
}
fn install_python() {} //for now this will do a global install 
fn install_git() {} //for now this will do a global install 
fn install_horde() {
    println!("works here.");
}
