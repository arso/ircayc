use std::collections::HashMap;

pub trait Executable {
    fn execute(&self) -> String;
}

pub trait Identifiable {
    fn id(&self) -> String;
}

pub struct CommandExecutor {
    executables_map: HashMap<String, Executable>
}

impl CommandExecutor {

    fn register (&self, executable : &Identifiable){
        self.executables_map.insert()
    }
}
