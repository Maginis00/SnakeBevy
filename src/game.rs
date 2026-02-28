use bevy::{app::{App, Plugin, Update}, ecs::schedule::{IntoScheduleConfigs, SystemSet}, state::{condition::in_state, state::{OnEnter, OnExit}}};

use crate::{GameState, logic, rendering};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Logic,
    Rendering,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            GameSet::Input,
            GameSet::Logic,
            GameSet::Rendering,
        ).chain().run_if(in_state(GameState::InGame)));

        app.add_systems(OnEnter(GameState::InGame), logic::setup_game);
        app.add_systems(OnExit(GameState::InGame), logic::cleanup_game);

        app.add_systems(Update, logic::snake_input.in_set(GameSet::Input));

        app.add_systems(Update, (
            logic::snake_movement,
            logic::snake_eating,
            logic::game_over_check,
        ).chain().in_set(GameSet::Logic));

        app.add_systems(Update, rendering::position_translation.in_set(GameSet::Rendering));
    }
}


