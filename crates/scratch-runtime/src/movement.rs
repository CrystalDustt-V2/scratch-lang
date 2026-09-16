use crate::world::World;

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
}
