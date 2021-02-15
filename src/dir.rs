use std::process::Command;

use log::debug;

use crate::actions::{Executable, Identifiable};

pub(crate) const LS_DIR_CMD: &'static str = "ls dir";
const EMPTY_RESP: &'static str = "";

pub struct ListDir { pub msg: String }

impl ListDir {}

impl Executable for ListDir {
    fn execute(&self) -> String {
        let msg_tokens: Vec<&str> = self.msg.split(LS_DIR_CMD).collect();
        let ls_path = msg_tokens[1].trim();

        return if !ls_path.is_empty() {
            debug!("DEBUG {} {}", msg_tokens[0], msg_tokens[1]);
            let dirs = Command::new("ls")
                .args(&["l", "a", &ls_path])
                .output()
                .expect("failed to execute process");
            debug!("ListDir cmd executed with code: {}", dirs.status.code().unwrap_or_default());
            let results = String::from_utf8(dirs.stdout).unwrap().replace("\n", ", ");
            debug!("ls dir: {}", results);
            results
        } else {
            debug!("DEBUG ls path not provided");
            EMPTY_RESP.to_string()
        };
    }
}

impl Identifiable for ListDir {
    fn id(&self) -> String {
        return LS_DIR_CMD.to_string();
    }
}
