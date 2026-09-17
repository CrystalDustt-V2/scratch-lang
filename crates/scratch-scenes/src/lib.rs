pub mod manager;
pub mod scene;

pub use manager::SceneManager;
pub use scene::{SceneCamera, SceneData, SceneError, SceneObject};

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_runtime::World;

    #[test]
    fn test_scene_roundtrip_and_application() {
        let mut scene = SceneData::default();
        scene.name = "Level1".into();
        scene.background = "forest".into();
        scene.objects.push(SceneObject {
            name: "Coin".into(),
            x: 100.0,
            y: 50.0,
            size: [20.0, 20.0],
            color: [1.0, 0.8, 0.0, 1.0],
            tags: vec!["collectible".into()],
            sprite: None,
        });

        let mut world = World::new();
        let mut manager = SceneManager::new();
        manager.register_scene(scene);

        manager.switch_scene("Level1", &mut world).expect("switch ok");

        assert_eq!(world.background, "forest");
        let player = world.get_entity_by_name("Player").expect("Player spawned");
        assert_eq!(player.transform.x, 0.0);

        let coin = world.get_entity_by_name("Coin").expect("Coin spawned");
        assert_eq!(coin.transform.x, 100.0);
        assert_eq!(coin.transform.y, 50.0);
        assert_eq!(coin.size, [20.0, 20.0]);
    }
}
