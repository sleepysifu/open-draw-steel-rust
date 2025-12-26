use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct EntityDefinition {
    pub name: String,
    pub max_stamina: i32,
}


pub struct Entity {
    instance_name: String,
    definition: EntityDefinition,
    current_stamina: i32,
}

impl Entity {
    pub fn new(instance_name: String, definition: EntityDefinition) -> Self {
        let current_stamina = definition.max_stamina;
        Self {
            instance_name,
            definition,
            current_stamina,
        }
    }

    pub fn from_definition_with_health(instance_name: String, definition: EntityDefinition, current_health: i32) -> Self {
        Self {
            instance_name,
            definition,
            current_stamina: current_health,
        }
    }

    pub fn definition(&self) -> &EntityDefinition {
        &self.definition
    }

    pub fn current_health(&self) -> i32 {
        self.current_stamina
    }

    pub fn max_health(&self) -> i32 {
        self.definition.max_stamina
    }

    pub fn name(&self) -> &String {
        &self.instance_name
    }

    pub fn definition_name(&self) -> &String {
        &self.definition.name
    }

    pub fn set_health(&mut self, health: i32) {
        self.current_stamina = health.max(0).min(self.definition.max_stamina);
    }

    pub fn damage(&mut self, amount: i32) {
        self.current_stamina = (self.current_stamina - amount).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.current_stamina = (self.current_stamina + amount).min(self.definition.max_stamina);
    }

    pub fn is_alive(&self) -> bool {
        self.current_stamina > 0
    }
}