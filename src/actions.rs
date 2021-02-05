pub trait Executable {
    fn execute(&self) -> String;
}

pub trait Identifiable {
    fn id(&self) -> String;
}
