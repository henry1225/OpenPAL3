use super::{map_role_id, SceneRoleExtensions};
use crate::director::sce_director::SceCommand;
use crate::director::sce_state::SceState;
use crate::scene::ScnScene;
use imgui::Ui;
use radiance::scene::CoreScene;

#[derive(Clone)]
pub struct SceCommandRoleActive {
    role_id: String,
    active: i32,
}

impl SceCommand for SceCommandRoleActive {
    fn initialize(&mut self, scene: &mut CoreScene<ScnScene>, state: &mut SceState) {}

    fn update(
        &mut self,
        scene: &mut CoreScene<ScnScene>,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        scene
            .get_role_entity(&self.role_id)
            .set_active(self.active != 0);
        true
    }
}

impl SceCommandRoleActive {
    pub fn new(role_id: i32, active: i32) -> Self {
        Self {
            role_id: map_role_id(role_id).to_string(),
            active,
        }
    }
}
