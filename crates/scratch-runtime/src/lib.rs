pub mod entity;
pub mod events;
pub mod executor;
pub mod list;
pub mod movement;
pub mod value;
pub mod world;

pub use entity::{Entity, EntityId, Transform2D};
pub use events::EventDispatcher;
pub use executor::Executor;
pub use list::ListSystem;
pub use movement::MovementSystem;
pub use value::RuntimeValue;
pub use world::World;

use scratch_blocks::BlockRegistry;
use scratch_ir::IrProgram;

pub struct Runtime {
    pub world: World,
    pub ir: IrProgram,
    pub registry: BlockRegistry,
}

impl Runtime {
    pub fn new(ir: IrProgram, registry: BlockRegistry) -> Self {
        let mut world = World::new();
        // Automatically spawn a default Player entity as per Section 52
        world.spawn_entity("Player");

        Self {
            world,
            ir,
            registry,
        }
    }

    pub fn start(&mut self) {
        EventDispatcher::dispatch_start(&self.ir, &mut self.world, &self.registry);
    }

    pub fn tick(&mut self, dt: f32) {
        // 0. Advance active movement tweens
        MovementSystem::tick_tweens(&mut self.world, dt);

        // 1. Process Input events
        EventDispatcher::dispatch_inputs(&self.ir, &mut self.world, &self.registry);

        // 2. Dispatch Collisions
        EventDispatcher::dispatch_collisions(&self.ir, &mut self.world, &self.registry);

        // 3. Dispatch Update
        EventDispatcher::dispatch_update(&self.ir, &mut self.world, &self.registry);

        // 4. Clear transient input states (pressed / up)
        self.world.clear_transient_inputs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_ir::lower_ast_to_ir;
    use scratch_language::parse;

    #[test]
    fn test_runtime_start_and_move() {
        let code = r#"
when start:
    score = 0

when action.down("right"):
    move(Player, 5)
"#;
        let ast = parse(code).expect("parse ok");
        let registry = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &registry).expect("lowering ok");

        let mut runtime = Runtime::new(ir, registry);

        // Verify start event sets score
        runtime.start();
        assert_eq!(runtime.world.get_var("score"), Some(&RuntimeValue::Number(0.0)));

        // Verify initial player position
        let player = runtime.world.get_entity_by_name("Player").expect("Player exists");
        assert_eq!(player.transform.x, 0.0);

        // Simulate pressing "right"
        runtime.world.set_action_down("right", true);
        runtime.tick(0.016);

        // Player should have moved 5 pixels
        let player = runtime.world.get_entity_by_name("Player").expect("Player exists");
        assert_eq!(player.transform.x, 5.0);

        // Tick again with "right" still held
        runtime.tick(0.016);
        let player = runtime.world.get_entity_by_name("Player").expect("Player exists");
        assert_eq!(player.transform.x, 10.0);

        // Release "right"
        runtime.world.set_action_down("right", false);
        runtime.tick(0.016);
        let player = runtime.world.get_entity_by_name("Player").expect("Player exists");
        assert_eq!(player.transform.x, 10.0);
    }

    #[test]
    fn test_collision_event() {
        let code = r#"
when start:
    score = 0

when Player touches Coin:
    collect(Coin)
    score += 1
"#;
        let ast = parse(code).expect("parse ok");
        let registry = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &registry).expect("lowering ok");

        let mut runtime = Runtime::new(ir, registry);
        runtime.world.spawn_entity("Coin");

        runtime.start();
        assert_eq!(runtime.world.get_var("score"), Some(&RuntimeValue::Number(0.0)));

        // Initially both Player and Coin are at (0, 0), touching!
        runtime.tick(0.016);

        // Score increased by 1 and Coin is now collected (hidden)
        assert_eq!(runtime.world.get_var("score"), Some(&RuntimeValue::Number(1.0)));
        let coin = runtime.world.get_entity_by_name("Coin").expect("Coin exists");
        assert!(!coin.visible);
    }
}
