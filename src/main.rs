//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // makes the terminal not show on windows //disabled for now

slint::include_modules!();

use std::process::Command;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Main_Window::new()?;
    main_window.on_install(|| {
        install();
    });

    main_window.run()
}

#[cfg(windows)]
fn check_for_deps_win() {
    //Checks for scoop, if scoop is installed, it will proceed, if not it will install it.
    let scoop_check = Command::new("scoop.cmd").arg("--version").output();

    match scoop_check {
        Ok(output) if output.status.success() => {
            println!("Scoop is installed. Proceeding.\n");
        }
        Ok(_) => {
            println!("Scoop was found but returned an error\n");
        }
        Err(e) => {
            println!("Scoop was not found, installing (Error: '{}')\n", e);
            install_scoop();
        }
    }
    //Checks for git, same deal as scoop
    let git_check = Command::new("git").arg("--version").output();

    match git_check {
        Ok(output) if output.status.success() => {
            println!("Git is installed. Proceeding.\n");
        }
        Ok(_) => {
            println!("Git was found but returned an error\n");
        }
        Err(e) => {
            println!("Git was not found, installing (Error: '{}')\n", e);
            install_git();
        }
    }
    //Checks for python, same deal as the rest
    let python_check = Command::new("python").arg("--version").output();

    match python_check {
        Ok(output) if output.status.success() => {
            println!("Python is installed. Proceeding.\n");
        }
        Ok(_) => {
            println!("Python was found but returned an error\n");
        }
        Err(e) => {
            println!("Python was not found, installing (Error: '{}')\n", e);
            install_python();
        }
    }
}

#[cfg(unix)]
fn check_for_deps_linux() {
    //get package manager, supported ones will be pacman, apt, and dnf
}
#[cfg(windows)]
fn install_scoop() {
    println!("Installing scoop");
}
fn install_python() {
    //for now this will do a global install
    println!("Installing Python");
}
fn install_git() {
    //for now this will do a global install
    println!("Installing git");
}
fn install_horde() {
    println!("Cloning ai horde");
    Command::new("git")
        .arg("clone")
        .arg("http://ryzen7-1700-a:3001/root/Learning-C.git")
        .arg("Horde-Image")
        .status()
        .expect("Something went wrong while trying clone the ai horde");
}
fn install() {
    #[cfg(windows)]
    check_for_deps_win();
    #[cfg(unix)]
    check_for_deps_linux();
    // install_horde()
}
