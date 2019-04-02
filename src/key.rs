use crate::color::Color;
use serde::{Deserialize, Serialize};

pub type KeyId = String;
pub type CategoryId = u8;

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: CategoryId,
    description: String,
    color: Color,
}

impl Category {
    pub fn new(id: CategoryId, description: &str, color: Color) -> Category {
        Category {
            id,
            description: description.to_owned(),
            color
        }
    }

    pub fn id(&self) -> CategoryId { self.id }
    pub fn description(&self) -> &str { &self.description }
    pub fn color(&self) -> Color { self.color }
}

#[derive(Serialize, Deserialize)]
pub struct Key {
    id: KeyId,
    category: CategoryId,
    custom_color: Option<Color>,
}

impl Key {
    pub fn new(id: KeyId, category: CategoryId, custom_color: Option<Color>) -> Self {
        Key {
            id,
            category,
            custom_color,
        }
    }

    pub fn id(&self) -> &KeyId {
        &self.id
    }

    pub fn category(&self) -> CategoryId {
        self.category
    }

    pub fn custom_color(&self) -> Option<Color> {
        self.custom_color
    }
}
