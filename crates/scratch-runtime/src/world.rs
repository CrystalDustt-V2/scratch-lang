use crate::entity::{Entity, EntityId};
use crate::value::RuntimeValue;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct World {
    entities: HashMap<EntityId, Entity>,
    name_index: HashMap<String, EntityId>,
    next_entity_id: u64,
    variables: HashMap<String, RuntimeValue>,
    pub input_actions_down: HashSet<String>,
    pub input_actions_pressed: HashSet<String>,
    pub input_actions_up: HashSet<String>,
    pub mouse_pos: (f32, f32),
    pub mouse_down: bool,
    pub background: String,
    pub camera_follow_target: Option<String>,
    pub camera_pos: (f32, f32),
    pub camera_zoom: f32,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            name_index: HashMap::new(),
            next_entity_id: 1,
            variables: HashMap::new(),
            input_actions_down: HashSet::new(),
            input_actions_pressed: HashSet::new(),
            input_actions_up: HashSet::new(),
            mouse_pos: (0.0, 0.0),
            mouse_down: false,
            background: "white".to_string(),
            camera_follow_target: None,
            camera_pos: (0.0, 0.0),
            camera_zoom: 1.0,
        }
    }

    pub fn spawn_entity(&mut self, name: impl Into<String>) -> EntityId {
        let name_str = name.into();
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;

        let entity = Entity::new(id, name_str.clone());
        self.entities.insert(id, entity);
        self.name_index.insert(name_str, id);
        id
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<&Entity> {
        self.name_index.get(name).and_then(|id| self.entities.get(id))
    }

    pub fn get_entity_by_name_mut(&mut self, name: &str) -> Option<&mut Entity> {
        if let Some(&id) = self.name_index.get(name) {
            self.entities.get_mut(&id)
        } else {
            None
        }
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn set_var(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.variables.insert(name.into(), value);
    }

    pub fn get_var(&self, name: &str) -> Option<&RuntimeValue> {
        self.variables.get(name)
    }

    pub fn get_var_or_nil(&self, name: &str) -> RuntimeValue {
        self.variables.get(name).cloned().unwrap_or(RuntimeValue::Nil)
    }

    pub fn is_action_down(&self, action: &str) -> bool {
        self.input_actions_down.contains(action)
    }

    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.input_actions_pressed.contains(action)
    }

    pub fn is_action_up(&self, action: &str) -> bool {
        self.input_actions_up.contains(action)
    }

    pub fn set_action_down(&mut self, action: &str, down: bool) {
        if down {
            if !self.input_actions_down.contains(action) {
                self.input_actions_pressed.insert(action.to_string());
            }
            self.input_actions_down.insert(action.to_string());
        } else {
            if self.input_actions_down.contains(action) {
                self.input_actions_up.insert(action.to_string());
            }
            self.input_actions_down.remove(action);
        }
    }

    pub fn clear_transient_inputs(&mut self) {
        self.input_actions_pressed.clear();
        self.input_actions_up.clear();
    }
}
