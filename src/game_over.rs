use crate::food::*;
use crate::snake::*;
use bevy::prelude::*;

pub struct GameOverEvent;

pub fn game_over_system(
    commands: &mut Commands,
    mut reader: Local<EventReader<GameOverEvent>>,
    game_over_events: Res<Events<GameOverEvent>>,
    segment_material: Res<SegmentMaterial>,
    head_material: Res<HeadMaterial>,
    segments: Query<(Entity, &SnakeSegment)>,
    food: Query<(Entity, &Food)>,
    heads: Query<(Entity, &SnakeHead)>,
) {
    if reader.iter(&game_over_events).next().is_some() {
        for (ent, _segment) in &mut segments.iter() {
            commands.despawn(ent);
        }
        for (ent, _food) in &mut food.iter() {
            commands.despawn(ent);
        }
        for (ent, _head) in &mut heads.iter() {
            commands.despawn(ent);
        }
        SnakeSegment::spawn_initial_snake(
            commands,
            head_material.0.clone(),
            segment_material.0.clone(),
        );
    }
}
