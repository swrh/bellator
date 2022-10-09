use std::time::Duration;

pub struct Entity {
}

impl Entity {
    pub fn new() -> Result<Entity, String> {
        Ok(Entity {
        })
    }

    pub fn update(&self, _instant: Duration) {
    }

    pub fn render(&self) {
    }
}
