// Walk through of:
// https://mbuffett.com/posts/bevy-snake-tutorial/
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;

mod common;
mod food;
mod game_over;
mod snake;

use common::{Size, *};
use food::*;
use game_over::*;
use snake::*;

// Pause
fn pause(mut paused: ResMut<Paused>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        paused.0 = !paused.0;
        println!("Pause: {}", paused.0)
    }
}

fn resource_setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Bevy requires a specific ordering to the params when registering systems.
    // Commands → Resources → Components/Queries.
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(HeadMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
    commands.insert_resource(SegmentMaterial(
        materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    ));
    commands.insert_resource(FoodMaterial(
        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    ));
    //commands
    //.spawn(UiCameraComponents::default())
    //.spawn(NodeComponents {
    //style: Style {
    //size: bevy::prelude::Size::new(Val::Px(100.0), Val::Px(100.0)),
    //..Default::default()
    //},
    //material: materials.add(Color::rgb(0.08, 0.08, 1.0).into()),
    //..Default::default()
    //});
}

fn game_setup(
    commands: Commands,
    head_material: Res<HeadMaterial>,
    segment_material: Res<SegmentMaterial>,
) {
    SnakeSegment::spawn_initial_snake(commands, head_material.0, segment_material.0);
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    if let Some(window) = windows.get_primary() {
        for (size, mut sprite) in &mut q.iter() {
            sprite.size = Vec2::new(
                size.width as f32 / ARENA_WIDTH as f32 * window.width as f32,
                size.height as f32 / ARENA_HEIGHT as f32 * window.width as f32,
            )
        }
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in &mut q.iter() {
            transform.set_translation(Vec3::new(
                convert(pos.x as f32, window.width as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height as f32, ARENA_HEIGHT as f32),
                0.0,
            ));
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ssssnake!".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(SnakeMoveTimer(Timer::new(
            Duration::from_millis((150.) as u64),
            true,
        )))
        .add_resource(FoodSpawnTimer(Timer::new(
            Duration::from_millis(1000),
            true,
        )))
        .add_event::<GameOverEvent>()
        .add_resource(common::Paused(false))
        .add_startup_system(resource_setup.system())
        .add_startup_stage("game_setup") // Not quite sure what Stage is doing here but lets keep going.
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(pause.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(food_spawner.system())
        .add_system(game_over_system.system())
        .add_default_plugins()
        .run();
}
