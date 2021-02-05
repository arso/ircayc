use std::process::Command;

use irc::client::prelude::*;

pub trait Executable {
    fn execute(&self) -> String;
}

pub struct ListDir { pub arguments: String, pub msg: Message }

pub struct OpenSSHTunnel {
    pub arguments: String
}

pub struct ConnectVPN {
    pub arguments: String
}

impl Executable for ListDir {
    fn execute(&self) -> String {
        let dirs = Command::new("ls")
            .args(&["l","a",&self.arguments])
            .output()
            .expect("failed to execute process");
        String::from_utf8(dirs.stdout).unwrap()
    }
}
