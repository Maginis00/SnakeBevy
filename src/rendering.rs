use bevy::prelude::*;
use crate::logic::{GameConfig, GameStateData, GridPosition, SnakeSegment, Apple};

// System: Syncs the logical position to the visual position
pub fn position_translation(
    config: Res<GameConfig>,
    mut query: Query<(&GridPosition, &mut Transform)>,
) {
    for (grid_pos, mut transform) in query.iter_mut() {
        // Umrechnung: Grid (z.B. 5) -> Pixel (z.B. 5 * 20.0 = 100.0)
        let x = (grid_pos.x as f32 * config.cell_size) + config.cell_size / 2.0;
        let y = (grid_pos.y as f32 * config.cell_size) + config.cell_size / 2.0;
        
        // In Bevy ist Y=0 in der Mitte. Wir wollen Y=0 unten links? 
        // Oder wir nutzen ein Kamera-Setup, das Y=0 oben links macht.
        // Für den Anfang: Einfach setzen.
        transform.translation = Vec3::new(x, y, 0.0);
    }
}

// System: Erstellt/Updated die Entities basierend auf dem Snake-Array
pub fn render_snake(
    mut commands: Commands,
    config: Res<GameConfig>,
    state: Res<GameStateData>,
    query: Query<Entity, With<SnakeSegment>>,
) {
    // Einfache Variante: Alte Entities löschen, neue spawnen.
    // (Performance für Snake okay, für große Spiele würde man "recyceln")
    
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for segment in state.snake.iter() {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.8, 0.1),
                custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
                ..default()
            },

            GridPosition { x: segment.0, y: segment.1 },
            SnakeSegment,
        ));
    }
}
