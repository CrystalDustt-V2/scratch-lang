use crate::executor::Executor;
use crate::world::World;
use scratch_blocks::BlockRegistry;
use scratch_ir::{IrProgram, IrTrigger};

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch_start(ir: &IrProgram, world: &mut World, registry: &BlockRegistry) {
        for handler in &ir.events {
            if matches!(handler.trigger, IrTrigger::OnStart) {
                Executor::execute_instructions(&handler.instructions, world, registry);
            }
        }
    }

    pub fn dispatch_update(ir: &IrProgram, world: &mut World, registry: &BlockRegistry) {
        for handler in &ir.events {
            if matches!(handler.trigger, IrTrigger::OnUpdate) {
                Executor::execute_instructions(&handler.instructions, world, registry);
            }
        }
    }

    pub fn dispatch_inputs(ir: &IrProgram, world: &mut World, registry: &BlockRegistry) {
        for handler in &ir.events {
            match &handler.trigger {
                IrTrigger::OnActionDown(action) => {
                    if world.is_action_down(action) {
                        Executor::execute_instructions(&handler.instructions, world, registry);
                    }
                }
                IrTrigger::OnActionPress(action) => {
                    if world.is_action_pressed(action) {
                        Executor::execute_instructions(&handler.instructions, world, registry);
                    }
                }
                IrTrigger::OnActionUp(action) => {
                    if world.is_action_up(action) {
                        Executor::execute_instructions(&handler.instructions, world, registry);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn dispatch_collisions(ir: &IrProgram, world: &mut World, registry: &BlockRegistry) {
        for handler in &ir.events {
            if let IrTrigger::OnTouches { object_a, object_b } = &handler.trigger {
                if let (Some(a), Some(b)) = (
                    world.get_entity_by_name(object_a),
                    world.get_entity_by_name(object_b),
                ) {
                    let half_w_a = a.size[0] / 2.0;
                    let half_h_a = a.size[1] / 2.0;
                    let half_w_b = b.size[0] / 2.0;
                    let half_h_b = b.size[1] / 2.0;

                    let overlap_x = (a.transform.x - b.transform.x).abs() <= (half_w_a + half_w_b);
                    let overlap_y = (a.transform.y - b.transform.y).abs() <= (half_h_a + half_h_b);

                    if overlap_x && overlap_y && a.visible && b.visible {
                        Executor::execute_instructions(&handler.instructions, world, registry);
                    }
                }
            }
        }
    }
}
