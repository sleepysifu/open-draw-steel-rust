#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PC {
    name: String,
}

impl PC {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
}