//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // makes the terminal not show on windows //disabled for now

slint::include_modules!();

use std::env;
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
            #[cfg(windows)]
            println!(
                "Python was found but returned an error, It is assumed that python is not actually installed, so installing with scoop.\n"
            );
            #[cfg(windows)]
            install_python();
            #[cfg(unix)]
            println!("Python was found but returned an error.\n");
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
    println!("Installing scoop\n");
    let scoop_install_script = r#"
        Set-Location C:\;
        Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression;
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") 
    "#;
    let scoop_install_command = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            scoop_install_script,
        ])
        .status()
        .expect("Failed to launch powershell");
    if scoop_install_command.success() {
        println!("Scoop installed successfully!");
    } else {
        println!("Failed to install Scoop.");
    }
}

fn install_python() {
    println!("Installing Python");
    let scoop_path = format!(
        r"{}\scoop\shims\scoop.cmd",
        env::var("USERPROFILE").unwrap()
    );
    let python_install_command = Command::new(scoop_path)
        .arg("install")
        .arg("python")
        .status()
        .expect("Failed to launch Scoop");
    if python_install_command.success() {
        println!("python installed successfully!");
    } else {
        println!("Failed to install Scoop.");
    }
}

fn install_git() {
    println!("Installing Git");
    let scoop_path = format!(
        r"{}\scoop\shims\scoop.cmd",
        env::var("USERPROFILE").unwrap()
    );
    let git_install_command = Command::new(scoop_path)
        .arg("install")
        .arg("git")
        .status()
        .expect("Failed to launch Scoop");
    if git_install_command.success() {
        println!("git installed successfully!");
    } else {
        println!("Failed to install Scoop.");
    }
}
fn install_horde() {
    println!("Cloning AI Horde ReGen");
    let git_path = format!(
        r"{}\scoop\apps\git\current\cmd\git.exe",
        env::var("USERPROFILE").unwrap()
    );
    Command::new(git_path)
        .arg("clone")
        .arg("https://github.com/Haidra-Org/horde-worker-reGen.git")
        .arg("Horde-Image")
        .status()
        .expect("Something went wrong while trying clone the ai horde");
}
fn install() {
    println!("Starting install");
    #[cfg(windows)]
    check_for_deps_win();
    #[cfg(unix)]
    check_for_deps_linux();
    install_horde();
    println!("\nInstall finished!");
}
