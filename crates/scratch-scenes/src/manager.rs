use crate::scene::{SceneData, SceneError};
use scratch_runtime::World;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SceneManager {
    scenes: HashMap<String, SceneData>,
    current_scene_name: String,
    scenes_dir: Option<PathBuf>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene_name: String::new(),
            scenes_dir: None,
        }
    }

    pub fn with_directory(mut self, path: impl AsRef<Path>) -> Self {
        self.scenes_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn register_scene(&mut self, scene: SceneData) {
        if self.current_scene_name.is_empty() {
            self.current_scene_name = scene.name.clone();
        }
        self.scenes.insert(scene.name.clone(), scene);
    }

    pub fn current_scene(&self) -> Option<&SceneData> {
        self.scenes.get(&self.current_scene_name)
    }

    pub fn switch_scene(&mut self, name: &str, world: &mut World) -> Result<(), SceneError> {
        // If not loaded in memory, try loading from scenes_dir
        if !self.scenes.contains_key(name) {
            if let Some(dir) = &self.scenes_dir {
                let candidate = dir.join(format!("{}.scene", name));
                if candidate.exists() {
                    let loaded = SceneData::load_from_file(&candidate)?;
                    self.scenes.insert(name.to_string(), loaded);
                } else {
                    return Err(SceneError::NotFound(name.to_string()));
                }
            } else {
                return Err(SceneError::NotFound(name.to_string()));
            }
        }

        self.current_scene_name = name.to_string();
        if let Some(scene) = self.scenes.get(name) {
            Self::apply_scene_to_world(scene, world);
        }

        Ok(())
    }

    pub fn restart_current_scene(&self, world: &mut World) {
        if let Some(scene) = self.scenes.get(&self.current_scene_name) {
            Self::apply_scene_to_world(scene, world);
        }
    }

    pub fn apply_scene_to_world(scene: &SceneData, world: &mut World) {
        world.background = scene.background.clone();
        world.camera_pos = (scene.camera.x, scene.camera.y);
        world.camera_zoom = scene.camera.zoom;
        world.camera_follow_target = scene.camera.follow.clone();

        for obj in &scene.objects {
            if let Some(ent) = world.get_entity_by_name_mut(&obj.name) {
                ent.transform.x = obj.x;
                ent.transform.y = obj.y;
                ent.size = obj.size;
                ent.color = obj.color;
                ent.tags = obj.tags.clone();
                ent.visible = true;
            } else {
                let id = world.spawn_entity(&obj.name);
                if let Some(ent) = world.get_entity_mut(id) {
                    ent.transform.x = obj.x;
                    ent.transform.y = obj.y;
                    ent.size = obj.size;
                    ent.color = obj.color;
                    ent.tags = obj.tags.clone();
                }
            }
        }
    }
}
