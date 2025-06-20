use super::*;

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Player {
    pub id: u8,
    pub speed: f32,
    pub animation_state: AnimationState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            speed: 1.0,
            animation_state: AnimationState::StandIdle,
        }
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub enum AnimationState {
    #[default]
    StandIdle,
    Run(f32),
    JumpStart,
    JumpLoop,
    JumpLand,
    Fall,
    CrouchWalk(f32),
    CrouchIdle,
    Dash,
    KnockBack,
}
