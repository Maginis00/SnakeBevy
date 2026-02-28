use bevy::prelude::*;

// --- Datenstrukturen ---
#[derive(Resource)]
pub struct GameConfig {
    pub grid_size: u32,   // z.B. 16 Felder breit/hoch
    pub cell_size: f32,   // z.B. 8.0 Pixel pro Feld
}

#[derive(Resource, Default)]
pub struct GameStateData {
    pub snake: Vec<(u32, u32)>, // Kopf ist am Index 0
    pub apple: (u32, u32),
    pub direction: Direction,
    pub next_move_timer: Timer, // Für die Geschwindigkeit
}

#[derive(Component)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Apple;

#[derive(Default, PartialEq)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

// --- Systeme ---

pub fn setup_game(mut commands: Commands) {
    // Ressourcen einfügen

    let config = GameConfig {
        grid_size: 16,
        cell_size: 20.,
    };

    let camera_x = (config.grid_size as f32 * config.cell_size) / 2.;
    let camera_y = (config.grid_size as f32 * config.cell_size) / 2.;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(camera_x, camera_y, 1000.)
    ));

    commands.insert_resource(config);
    commands.insert_resource(GameStateData {
        snake: vec![(5, 5)], // Startposition
        apple: (10, 10),
        next_move_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        ..default()
    });
}

pub fn cleanup_game(mut commands: Commands) {
    // Alle Spiel-Entities entfernen beim Verlassen
    commands.remove_resource::<GameConfig>();
    commands.remove_resource::<GameStateData>();
}

pub fn snake_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameStateData>,
) {
    if keyboard.pressed(KeyCode::ArrowUp) && state.direction != Direction::Down {
        state.direction = Direction::Up;
    }
    if keyboard.pressed(KeyCode::ArrowDown) && state.direction != Direction::Up {
        state.direction = Direction::Down;
    }
    // ... Links/Rechts analog
}

pub fn snake_movement(
    time: Res<Time>,
    mut state: ResMut<GameStateData>,
    config: Res<GameConfig>,
) {
    // Timer runterzählen
    if state.next_move_timer.tick(time.delta()).just_finished() {
        // Bewegungslogik hier
        let head = state.snake.first().unwrap();
        let new_head = match state.direction {
            Direction::Up => (head.0, head.1.wrapping_add(1) % config.grid_size),
            Direction::Down => (head.0, head.1.wrapping_sub(1) % config.grid_size),
            // ...
            _ => *head,
        };
        state.snake.insert(0, new_head);
        state.snake.pop(); // Schwanz entfernen (wenn nicht gegessen)
    }
}

// Platzhalter für andere Logik
pub fn snake_eating() {}
pub fn game_over_check() {}

