use pumpkin::{
    entity::{
        ai::{
            brain::memory::BlockPosTracker,
            goal::{Controls, Goal},
        },
        mob::Mob,
    },
    plugin::{
        EventHandler,
        loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::event::PlayerInteractEventData,
    },
    world::World,
};
use pumpkin_util::math::position::BlockPos;

#[derive(Default)]
pub struct TargetContainerGoal {
    target: Option<BlockPos>,
    search_radius: f64,
}

pub struct WitnessPlayerInteraction;

impl TargetContainerGoal {
    fn new() -> Self {
        Self {
            search_radius: 16.0,
            ..Default::default()
        }
    }
}

impl Goal for TargetContainerGoal {
    fn can_start(&mut self, _mob: &dyn Mob) -> bool {
        let eye_p = _mob.get_eye_pos();
        false
    }

    fn should_continue(&mut self, mob: &dyn Mob) -> bool {
        self.can_start(mob)
    }

    fn start(&mut self, _mob: &dyn Mob) {}

    fn stop(&mut self, _mob: &dyn Mob) {}

    fn tick(&mut self, _mob: &dyn Mob) {}

    fn should_run_every_tick(&self) -> bool {
        false
    }

    fn can_stop(&self) -> bool {
        true
    }

    fn get_tick_count(&self, ticks: i32) -> i32 {
        if self.should_run_every_tick() {
            ticks
        } else {
            pumpkin::entity::ai::goal::to_goal_ticks(ticks)
        }
    }

    fn controls(&self) -> Controls {
        Controls::empty()
    }
}
