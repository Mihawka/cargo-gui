use std::process::Command;

pub struct Project {
    pub name: String,
    pub directory: String,
    pub is_lib: bool,
}

impl Project {
    pub fn create_project(&self) {
        let dir = format!("{}/{}", self.directory, self.name);
        let lib = match self.is_lib {
            true => "--lib",
            false => "--bin",
        };
        
        let output = Command::new("cargo")
            .args(&["new", dir.as_str(), lib])
            .output()
            .unwrap();
    }
}
