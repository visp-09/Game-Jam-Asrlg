use bracket_terminal::{
    prelude::{ColorPair, Point},
    FontCharType,
};
use legion::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub last_point: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerParryCooldown {
    pub timestamp: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dead;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HasLineOfSight;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
    pub damage: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pub initial: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DetonatorAI {
    pub tile_length: i32,
    pub dimensions: Point,
    pub interval: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RippleAI {
    pub max_radius: i32,
    pub current_radius: i32,
    pub timestamp: u64,
    pub interval: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Projectile {
    pub lifetime: i32,
    pub damage: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAttackStage {
    Ready,
    Stage1,
    Stage2,
    Stage3,
    Stage4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerAttackStatus {
    pub stage: PlayerAttackStage,
    pub stage_timer: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAttackType {
    RightFront,
    LeftFront,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timer {
    pub time_stamp: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parrying {
    pub timer: i32
}
