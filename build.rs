#![allow(dead_code)]

use std::{env, fs, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui_working_dir = env::current_dir()?;
    ui_working_dir.push("src");
    ui_working_dir.push("ui");

    let profile = env::var("PROFILE")?;
    match profile.as_ref() {
        "debug" => {
            println!("cargo:warning=Running in debug mode, please start the development server in another terminal instead.");

            let mut dist_path = PathBuf::from(&ui_working_dir);
            dist_path.push("dist");

            if !dist_path.exists() {
                fs::create_dir(&dist_path).unwrap();
            } else {
                println!("{:?} already exists", dist_path.as_os_str());
            }

            let mut index_html_path = PathBuf::from(&dist_path);
            index_html_path.push("index.html");

            if !index_html_path.exists() {
                fs::File::create(index_html_path).unwrap();
            } else {
                println!("{:?} already exists", index_html_path.as_os_str());
            }
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
