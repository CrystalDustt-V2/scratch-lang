use scratch_project::ProjectConfig;
use scratch_runtime::Runtime;

pub struct NativeRunner;

#[cfg(feature = "bevy-engine")]
mod bevy_adapter {
    use super::*;
    use bevy::prelude::*;
    use scratch_runtime::EntityId;
    use std::collections::HashMap;

    #[derive(Resource)]
    pub struct RuntimeResource {
        pub runtime: Runtime,
    }

    #[derive(Resource)]
    pub struct BevyEntityMap {
        pub map: HashMap<EntityId, Entity>,
    }

    pub fn run_bevy(runtime: Runtime, config: &ProjectConfig) {
        let title = config.name.clone();
        let width = if config.resolution.width > 0 { config.resolution.width as f32 } else { 1280.0 };
        let height = if config.resolution.height > 0 { config.resolution.height as f32 } else { 720.0 };

        let mut app = App::new();

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title,
                resolution: (width, height).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }));

        app.insert_resource(ClearColor(Color::WHITE));
        app.insert_resource(RuntimeResource { runtime });
        app.insert_resource(BevyEntityMap {
            map: HashMap::new(),
        });

        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, (handle_input, tick_runtime, sync_entities).chain());

        app.run();
    }

    fn setup_scene(mut commands: Commands, mut res: ResMut<RuntimeResource>) {
        // Spawn 2D Camera centered on scene
        if res.runtime.world.is_corner_mode() {
            let cx = res.runtime.world.stage_width / 2.0;
            let cy = res.runtime.world.stage_height / 2.0;
            commands.spawn((
                Camera2d,
                Transform::from_xyz(cx, cy, 0.0),
            ));
        } else {
            commands.spawn(Camera2d);
        }

        // Run OnStart events in runtime
        res.runtime.start();
    }

    fn handle_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut res: ResMut<RuntimeResource>,
    ) {
        let world = &mut res.runtime.world;

        // Map keyboard keys to logical actions (Up decoupled from Jump)
        let right = keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight);
        let left = keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft);
        let up = keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp);
        let down = keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown);
        let jump = keyboard.pressed(KeyCode::Space);

        world.set_action_down("right", right);
        world.set_action_down("left", left);
        world.set_action_down("up", up);
        world.set_action_down("down", down);
        world.set_action_down("jump", jump);

        world.set_action_down("w", keyboard.pressed(KeyCode::KeyW));
        world.set_action_down("s", keyboard.pressed(KeyCode::KeyS));
        world.set_action_down("a", keyboard.pressed(KeyCode::KeyA));
        world.set_action_down("d", keyboard.pressed(KeyCode::KeyD));
        world.set_action_down("ArrowUp", keyboard.pressed(KeyCode::ArrowUp));
        world.set_action_down("ArrowDown", keyboard.pressed(KeyCode::ArrowDown));
        world.set_action_down("ArrowLeft", keyboard.pressed(KeyCode::ArrowLeft));
        world.set_action_down("ArrowRight", keyboard.pressed(KeyCode::ArrowRight));
        world.set_action_down("space", jump);
    }

    fn tick_runtime(time: Res<Time>, mut res: ResMut<RuntimeResource>) {
        res.runtime.tick(time.delta_secs());
    }

    fn sync_entities(
        mut commands: Commands,
        mut entity_map: ResMut<BevyEntityMap>,
        res: Res<RuntimeResource>,
        mut transforms: Query<&mut Transform>,
    ) {
        let world = &res.runtime.world;

        for ent in world.iter_entities() {
            if !ent.visible {
                if let Some(bevy_id) = entity_map.map.remove(&ent.id) {
                    commands.entity(bevy_id).despawn();
                }
                continue;
            }

            if let Some(&bevy_id) = entity_map.map.get(&ent.id) {
                if let Ok(mut transform) = transforms.get_mut(bevy_id) {
                    transform.translation.x = ent.transform.x;
                    transform.translation.y = ent.transform.y;
                }
            } else {
                // Spawn new sprite for entity
                let color = Color::srgb(ent.color[0], ent.color[1], ent.color[2]);
                let size = Vec2::new(ent.size[0], ent.size[1]);

                let spawned = commands
                    .spawn((
                        Sprite {
                            color,
                            custom_size: Some(size),
                            ..default()
                        },
                        Transform::from_xyz(ent.transform.x, ent.transform.y, 0.0),
                    ))
                    .id();

                entity_map.map.insert(ent.id, spawned);
            }
        }
    }
}

impl NativeRunner {
    pub fn run(#[allow(unused_mut)] mut runtime: Runtime, config: &ProjectConfig) {
        #[cfg(feature = "bevy-engine")]
        {
            bevy_adapter::run_bevy(runtime, config);
        }

        #[cfg(not(feature = "bevy-engine"))]
        {
            println!("Starting scratch-lang native runner for '{}'...", config.name);
            println!("Target resolution: {}x{}", config.resolution.width, config.resolution.height);
            runtime.start();
            println!("Game initialized. Runtime started successfully.");
            println!("(Running in lightweight headless mode. Compile with '--features bevy-engine' for graphical window)");
        }
    }

    pub fn run_headless_ticks(runtime: &mut Runtime, ticks: usize, dt: f32) {
        runtime.start();
        for _ in 0..ticks {
            runtime.tick(dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_blocks::BlockRegistry;
    use scratch_ir::lower_ast_to_ir;
    use scratch_language::parse;

    #[test]
    fn test_headless_runner_ticks() {
        let code = r#"
when start:
    score = 10

when update:
    score += 1
"#;
        let ast = parse(code).expect("parse");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir");
        let mut runtime = Runtime::new(ir, reg);

        NativeRunner::run_headless_ticks(&mut runtime, 5, 0.016);
        assert_eq!(runtime.world.get_var("score").unwrap().as_number(), Some(15.0));
    }
}
