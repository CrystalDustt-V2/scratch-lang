use crate::vm::{Vm, VmError};
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeProgram;
use scratch_ir::IrTrigger;
use scratch_runtime::World;

pub struct VmRuntime {
    pub world: World,
    pub bytecode: BytecodeProgram,
    pub registry: BlockRegistry,
    pub vm: Vm,
}

impl VmRuntime {
    pub fn new(bytecode: BytecodeProgram, registry: BlockRegistry) -> Self {
        let mut world = World::new();
        world.spawn_entity("Player");

        Self {
            world,
            bytecode,
            registry,
            vm: Vm::default(),
        }
    }

    pub fn start(&mut self) -> Result<(), VmError> {
        for event in &self.bytecode.events {
            if matches!(event.trigger, IrTrigger::OnStart) {
                self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
            }
        }
        Ok(())
    }

    pub fn tick(&mut self, _dt: f32) -> Result<(), VmError> {
        // 1. Inputs
        for event in &self.bytecode.events {
            match &event.trigger {
                IrTrigger::OnActionDown(action) => {
                    if self.world.is_action_down(action) {
                        self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                    }
                }
                IrTrigger::OnActionPress(action) => {
                    if self.world.is_action_pressed(action) {
                        self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                    }
                }
                IrTrigger::OnActionUp(action) => {
                    if self.world.is_action_up(action) {
                        self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                    }
                }
                _ => {}
            }
        }

        // 2. Collisions
        for event in &self.bytecode.events {
            if let IrTrigger::OnTouches { object_a, object_b } = &event.trigger {
                if let (Some(a), Some(b)) = (
                    self.world.get_entity_by_name(object_a),
                    self.world.get_entity_by_name(object_b),
                ) {
                    let half_w_a = a.size[0] / 2.0;
                    let half_h_a = a.size[1] / 2.0;
                    let half_w_b = b.size[0] / 2.0;
                    let half_h_b = b.size[1] / 2.0;

                    let overlap_x = (a.transform.x - b.transform.x).abs() <= (half_w_a + half_w_b);
                    let overlap_y = (a.transform.y - b.transform.y).abs() <= (half_h_a + half_h_b);

                    if overlap_x && overlap_y && a.visible && b.visible {
                        self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                    }
                }
            }
        }

        // 3. Update
        for event in &self.bytecode.events {
            if matches!(event.trigger, IrTrigger::OnUpdate) {
                self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
            }
        }

        // 4. Clear transient input states
        self.world.clear_transient_inputs();

        Ok(())
    }
}
