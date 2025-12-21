#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NPC {
    name: String,
}

impl NPC {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
}