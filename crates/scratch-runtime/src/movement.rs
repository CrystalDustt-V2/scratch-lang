use crate::entity::RotationStyle;
use crate::value::RuntimeValue;
use crate::world::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlideTween {
    pub target: String,
    pub start_x: f32,
    pub start_y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub duration: f32,
    pub elapsed: f32,
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn execute_move(world: &mut World, target: &str, dx: f32, dy: f32) {
        if let Some(entity) = world.get_entity_by_name_mut(target) {
            entity.transform.x += dx;
            entity.transform.y += dy;
        }
    }

    pub fn execute_jump(world: &mut World, target: &str, force: f32) {
        if let Some(entity) = world.get_entity_by_name_mut(target) {
            entity.velocity.1 = force;
            entity.transform.y += force;
        }
    }

    pub fn execute_stop(world: &mut World, target: &str) {
        if let Some(entity) = world.get_entity_by_name_mut(target) {
            entity.velocity = (0.0, 0.0);
        }
    }

    pub fn execute_teleport(world: &mut World, target: &str, x: f32, y: f32) {
        if let Some(entity) = world.get_entity_by_name_mut(target) {
            entity.transform.x = x;
            entity.transform.y = y;
        }
    }

    pub fn start_glide(world: &mut World, target: &str, duration: f32, target_x: f32, target_y: f32) {
        if let Some(entity) = world.get_entity_by_name(target) {
            let start_x = entity.transform.x;
            let start_y = entity.transform.y;
            world.active_tweens.retain(|t| t.target != target);
            world.active_tweens.push(GlideTween {
                target: target.to_string(),
                start_x,
                start_y,
                target_x,
                target_y,
                duration: duration.max(0.001),
                elapsed: 0.0,
            });
        }
    }

    pub fn tick_tweens(world: &mut World, dt: f32) {
        let tweens = std::mem::take(&mut world.active_tweens);
        let mut remaining = Vec::with_capacity(tweens.len());
        for mut tween in tweens {
            tween.elapsed += dt;
            let progress = (tween.elapsed / tween.duration).clamp(0.0, 1.0);
            let cur_x = tween.start_x + (tween.target_x - tween.start_x) * progress;
            let cur_y = tween.start_y + (tween.target_y - tween.start_y) * progress;
            if let Some(ent) = world.get_entity_by_name_mut(&tween.target) {
                ent.transform.x = cur_x;
                ent.transform.y = cur_y;
            }
            if progress < 1.0 {
                remaining.push(tween);
            }
        }
        remaining.extend(std::mem::take(&mut world.active_tweens));
        world.active_tweens = remaining;
    }

    pub fn execute_go_to(world: &mut World, target: &str, destination: &str) {
        let dest_pos = match destination.to_lowercase().as_str() {
            "mouse" | "mouse-pointer" | "mouse_pointer" => Some(world.mouse_pos),
            "random" | "random position" | "random_position" => {
                let seed = world
                    .get_var("__random_seed")
                    .map(|v| match v {
                        RuntimeValue::Number(n) => *n,
                        _ => 54321.0,
                    })
                    .unwrap_or(54321.0);
                let next_x = (seed * 1103515245.0 + 12345.0) % 2147483648.0;
                let next_y = (next_x * 1103515245.0 + 12345.0) % 2147483648.0;
                world.set_var("__random_seed", RuntimeValue::Number(next_y));
                let rx = 50.0 + (next_x / 2147483648.0) as f32 * (1280.0 - 100.0);
                let ry = 50.0 + (next_y / 2147483648.0) as f32 * (720.0 - 100.0);
                Some((rx, ry))
            }
            _ => world
                .get_entity_by_name(destination)
                .or_else(|| {
                    world
                        .iter_entities()
                        .find(|e| e.name.eq_ignore_ascii_case(destination))
                })
                .map(|e| (e.transform.x, e.transform.y)),
        };

        if let Some((x, y)) = dest_pos {
            if let Some(ent) = world.get_entity_by_name_mut(target) {
                ent.transform.x = x;
                ent.transform.y = y;
            }
        }
    }

    pub fn execute_set_rotation_style(world: &mut World, target: &str, style_str: &str) {
        let style = match style_str.to_lowercase().as_str() {
            "left-right" | "left_right" | "leftright" => RotationStyle::LeftRight,
            "don't rotate" | "dont_rotate" | "none" => RotationStyle::None,
            _ => RotationStyle::AllAround,
        };
        if let Some(ent) = world.get_entity_by_name_mut(target) {
            ent.rotation_style = style;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_to_destinations() {
        let mut world = World::new();
        world.spawn_entity("Player");
        world.spawn_entity("Target");
        if let Some(t) = world.get_entity_by_name_mut("Target") {
            t.transform.x = 420.0;
            t.transform.y = 690.0;
        }

        // Test go_to mouse
        world.mouse_pos = (150.0, 300.0);
        MovementSystem::execute_go_to(&mut world, "Player", "mouse");
        let p = world.get_entity_by_name("Player").unwrap();
        assert_eq!(p.transform.x, 150.0);
        assert_eq!(p.transform.y, 300.0);

        // Test go_to entity
        MovementSystem::execute_go_to(&mut world, "Player", "Target");
        let p = world.get_entity_by_name("Player").unwrap();
        assert_eq!(p.transform.x, 420.0);
        assert_eq!(p.transform.y, 690.0);

        // Test go_to random
        MovementSystem::execute_go_to(&mut world, "Player", "random");
        let p = world.get_entity_by_name("Player").unwrap();
        assert!(p.transform.x >= 50.0 && p.transform.x <= 1280.0);
        assert!(p.transform.y >= 50.0 && p.transform.y <= 720.0);
    }

    #[test]
    fn test_set_rotation_style() {
        let mut world = World::new();
        world.spawn_entity("Player");

        MovementSystem::execute_set_rotation_style(&mut world, "Player", "left-right");
        assert_eq!(world.get_entity_by_name("Player").unwrap().rotation_style, RotationStyle::LeftRight);

        MovementSystem::execute_set_rotation_style(&mut world, "Player", "don't rotate");
        assert_eq!(world.get_entity_by_name("Player").unwrap().rotation_style, RotationStyle::None);

        MovementSystem::execute_set_rotation_style(&mut world, "Player", "all-around");
        assert_eq!(world.get_entity_by_name("Player").unwrap().rotation_style, RotationStyle::AllAround);
    }

    #[test]
    fn test_glide_tween_interpolation() {
        let mut world = World::new();
        world.spawn_entity("Player");

        // Start glide from (0, 0) to (100, 200) over 2.0 seconds
        MovementSystem::start_glide(&mut world, "Player", 2.0, 100.0, 200.0);
        assert_eq!(world.active_tweens.len(), 1);

        // Advance 1.0 second (50% progress)
        MovementSystem::tick_tweens(&mut world, 1.0);
        let p = world.get_entity_by_name("Player").unwrap();
        assert!((p.transform.x - 50.0).abs() < 0.001);
        assert!((p.transform.y - 100.0).abs() < 0.001);
        assert_eq!(world.active_tweens.len(), 1);

        // Advance another 1.0 second (100% progress -> complete)
        MovementSystem::tick_tweens(&mut world, 1.0);
        let p = world.get_entity_by_name("Player").unwrap();
        assert!((p.transform.x - 100.0).abs() < 0.001);
        assert!((p.transform.y - 200.0).abs() < 0.001);
        assert_eq!(world.active_tweens.len(), 0);
    }
}
