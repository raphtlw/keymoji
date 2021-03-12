use std::process::{Command, Stdio};

pub enum Shell {}

impl Shell {
    pub fn cmd(cmd: String) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run shell command");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
