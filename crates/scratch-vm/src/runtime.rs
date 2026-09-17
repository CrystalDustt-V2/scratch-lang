use crate::vm::{Vm, VmError};
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeProgram;
use scratch_ir::IrTrigger;
use scratch_runtime::{RuntimeValue, World};
use scratch_scenes::SceneManager;

#[derive(Debug, Clone)]
pub struct TimerTracker {
    pub seconds: f32,
    pub repeating: bool,
    pub elapsed: f32,
    pub fired: bool,
}

pub struct VmRuntime {
    pub world: World,
    pub bytecode: BytecodeProgram,
    pub registry: BlockRegistry,
    pub vm: Vm,
    pub scene_manager: Option<SceneManager>,
    pub timers: Vec<TimerTracker>,
}

impl VmRuntime {
    pub fn new(bytecode: BytecodeProgram, registry: BlockRegistry) -> Self {
        let mut world = World::new();
        world.spawn_entity("Player");

        let mut timers = Vec::new();
        for event in &bytecode.events {
            if let IrTrigger::OnTimer { seconds, repeating } = &event.trigger {
                timers.push(TimerTracker {
                    seconds: *seconds as f32,
                    repeating: *repeating,
                    elapsed: 0.0,
                    fired: false,
                });
            }
        }

        Self {
            world,
            bytecode,
            registry,
            vm: Vm::default(),
            scene_manager: None,
            timers,
        }
    }

    pub fn with_scene_manager(mut self, manager: SceneManager) -> Self {
        self.scene_manager = Some(manager);
        self
    }

    pub fn start(&mut self) -> Result<(), VmError> {
        for event in &self.bytecode.events {
            if matches!(event.trigger, IrTrigger::OnStart) {
                self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
            }
        }
        self.dispatch_messages()?;
        self.check_scene_signals();
        Ok(())
    }

    pub fn tick(&mut self, dt: f32) -> Result<(), VmError> {
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

        // 3. Timers
        let mut timer_idx = 0;
        for event in &self.bytecode.events {
            if let IrTrigger::OnTimer { .. } = &event.trigger {
                if let Some(timer) = self.timers.get_mut(timer_idx) {
                    timer.elapsed += dt;
                    if (!timer.fired || timer.repeating) && timer.elapsed >= timer.seconds {
                        self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                        if timer.repeating {
                            timer.elapsed = 0.0;
                        } else {
                            timer.fired = true;
                        }
                    }
                }
                timer_idx += 1;
            }
        }

        // 4. Update
        for event in &self.bytecode.events {
            if matches!(event.trigger, IrTrigger::OnUpdate) {
                self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
            }
        }

        // 5. Broadcast Messages
        self.dispatch_messages()?;

        // 6. Check Scene Transitions
        self.check_scene_signals();

        // 7. Clear transient input states
        self.world.clear_transient_inputs();

        Ok(())
    }

    pub fn dispatch_messages(&mut self) -> Result<(), VmError> {
        let mut cascade_guard = 0;
        while !self.world.pending_messages.is_empty() && cascade_guard < 50 {
            cascade_guard += 1;
            let messages = self.world.take_messages();
            for msg in messages {
                for event in &self.bytecode.events {
                    if let IrTrigger::OnMessage(target_msg) = &event.trigger {
                        if target_msg == &msg {
                            self.vm.execute(&event.chunk, &mut self.world, &self.registry)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn check_scene_signals(&mut self) {
        if let Some(scene_val) = self.world.get_var("__next_scene").cloned() {
            self.world.set_var("__next_scene", RuntimeValue::Nil);
            if let Some(scene_name) = scene_val.as_string() {
                if let Some(mgr) = &mut self.scene_manager {
                    let _ = mgr.switch_scene(scene_name, &mut self.world);
                }
            }
        }

        if let Some(restart_val) = self.world.get_var("__restart_scene").cloned() {
            if restart_val.as_bool() {
                self.world.set_var("__restart_scene", RuntimeValue::Bool(false));
                if let Some(mgr) = &self.scene_manager {
                    mgr.restart_current_scene(&mut self.world);
                }
            }
        }
    }
}
