use super::UpdateState;

use crate::render::render_state::RenderState;
use crate::IOScheduler;
use std::time::Duration;

pub struct PinchHandler {}

impl UpdateState for PinchHandler {
    fn update_state(&mut self, _state: &mut RenderState, _scheduler: &IOScheduler, _dt: Duration) {
        // TODO
    }
}

impl PinchHandler {
    pub fn new() -> Self {
        Self {}
    }
}
