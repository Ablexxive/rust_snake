// Initial gameplay code from:
// https://mbuffett.com/posts/bevy-snake-tutorial/
use bevy::app::startup_stage::PRE_STARTUP;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;

mod common;
mod food;
mod game_over;
mod pause_screen;
mod snake;

use common::{Size, *};
use food::*;
use game_over::*;
use pause_screen::*;
use snake::*;

fn resource_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(HeadMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
    commands.insert_resource(SegmentMaterial(
        materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    ));
    commands.insert_resource(FoodMaterial(
        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    ));

    // Pause Menu Elements
    commands.spawn(CameraUiBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: bevy::prelude::Size::new(Val::Px(2000.0), Val::Px(2000.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            draw: Draw::default(),
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(PauseScreenItem)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        size: bevy::prelude::Size::new(Val::Px(200.0), Val::Px(200.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "Pause".to_string(),
                        font: asset_server.load("fonts/SFNS.ttf"),
                        style: TextStyle {
                            font_size: 200.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    draw: Draw::default(),
                    visible: Visible {
                        is_visible: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(PauseScreenItem);
        });
}

fn game_setup(
    commands: &mut Commands,
    head_material: Res<HeadMaterial>,
    segment_material: Res<SegmentMaterial>,
) {
    SnakeSegment::spawn_initial_snake(
        commands,
        head_material.0.clone(),
        segment_material.0.clone(),
    );
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Sprite)>) {
    if let Some(window) = windows.get_primary() {
        for (size, mut sprite) in query.iter_mut() {
            sprite.size = Vec2::new(
                size.width as f32 / ARENA_WIDTH as f32 * window.width() as f32,
                size.height as f32 / ARENA_HEIGHT as f32 * window.height() as f32,
            )
        }
    }
}

fn position_translation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
    fn convert(p: f32, bound_window: f32, bound_game: f32) -> f32 {
        p / bound_game * bound_window - (bound_window / 2.)
    }
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in query.iter_mut() {
            *transform = Transform::from_translation(Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0,
            ));
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ssssnake!".to_string(),
            width: 2000.0,
            height: 2000.0,
            ..Default::default()
        })
        .add_resource(common::Paused(false))
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(SnakeMoveTimer(Timer::new(
            Duration::from_millis((150.) as u64),
            true,
        )))
        .add_resource(FoodSpawnTimer(Timer::new(
            Duration::from_millis(1000),
            true,
        )))
        .add_startup_system_to_stage(PRE_STARTUP, resource_setup.system())
        .add_startup_system(game_setup.system())
        .add_event::<GameOverEvent>()
        .add_system(pause.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(food_spawner.system())
        .add_system(game_over_system.system())
        .add_plugins(DefaultPlugins)
        .run();
}
