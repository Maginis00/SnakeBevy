use bevy::prelude::*;

use crate::game::GamePlugin;

mod game;
mod logic;
mod rendering;

// Main (test)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(GamePlugin)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    InMenu,
    InGame,
    GameOver,
}
