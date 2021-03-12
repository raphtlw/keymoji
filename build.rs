#![allow(dead_code)]

use std::{env, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui_working_dir = env::current_dir()?;
    ui_working_dir.push("src");
    ui_working_dir.push("ui");

    let profile = env::var("PROFILE")?;
    match profile.as_ref() {
        "debug" => {
            println!("cargo:warning=Running in debug mode, please start the development server in another terminal instead.")
        }
        "release" => {
            let mut build_process = Command::new("npm")
                .args(&["run", "build"])
                .current_dir(ui_working_dir)
                .spawn()?;

            build_process.wait()?;
        }
        _ => unimplemented!(),
    }

    Ok(())
}
enum Shell {}

impl Shell {
    fn cmd(cmd: String) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to run shell command");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
