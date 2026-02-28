use bevy::prelude::*;

use crate::game::GamePlugin;

mod game;
mod logic;
mod rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>
        .add_plugins(GamePlugin)
        .run();
}

#[derive(States, Default, )]
pub enum GameState {
    #[default],
    InMenu,
    InGame,
    GameOver,
}
