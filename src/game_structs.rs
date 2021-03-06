use tetra::math::Vec2;
use tetra::graphics::animation::Animation;
use tetra::graphics::{Texture, Camera};

pub struct Tile {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub collidable: bool,
}

pub struct Player {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity_x: f32,
    pub colliding: bool,
    pub facing: i8,
    pub prev_facing: i8,
    pub alive: bool,
}

pub struct PlayerAttackSphere {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity: f32,
    pub facing: i8,
    pub visible: bool,
}

pub struct Enemy {
    pub animation: Animation,
    pub position: Vec2<f32>,
    pub velocity: f32,
    pub range_end: f32,
    pub range_start: f32,
    pub facing: i8,
}

pub struct Npc {
    pub animation: Animation,
    pub position: Vec2<f32>,
}

pub struct Help_Menu {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub visible: bool,
    pub text: String,
}

pub struct DialogueBox {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub visible: bool,
    pub text: String,
    pub output_text: String,
    pub text_wrap_finished: bool,
}

pub struct GameState {
    pub player: Player,
    pub npc: Npc,
    pub tiles: Vec<Tile>,
    pub player_attack_instances: Vec<PlayerAttackSphere>,
    pub enemy_instances: Vec<Enemy>,
    pub camera: Camera,
    pub help_menu: Help_Menu,
    pub dialogue_box: DialogueBox,
}
