use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub tags: Vec<String>,
    pub transform: Transform2D,
    pub visible: bool,
    pub color: [f32; 4],
    pub size: [f32; 2],
    pub health: f64,
    pub velocity: (f32, f32),
}

impl Entity {
    pub fn new(id: EntityId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            tags: Vec::new(),
            transform: Transform2D::default(),
            visible: true,
            color: [0.2, 0.6, 1.0, 1.0], // default friendly blue color
            size: [40.0, 40.0],
            health: 3.0,
            velocity: (0.0, 0.0),
        }
    }
}
