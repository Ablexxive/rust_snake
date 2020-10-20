// Walk through of:
// https://mbuffett.com/posts/bevy-snake-tutorial/
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;

const ARENA_WIDTH: u32 = 40;
const ARENA_HEIGHT: u32 = 40;

// Snake Head Information
struct SnakeMoveTimer(Timer);
struct SnakeHead {
    direction: Direction,
    next_segment: Entity,
}
struct HeadMaterial(Handle<ColorMaterial>); // resource for storing snake head sprite

// Snake Tail Info
#[derive(Default)]
struct SnakeSegment {
    next_segment: Option<Entity>,
}
struct SegmentMaterial(Handle<ColorMaterial>);

impl SnakeSegment {
    fn spawn_segment(commands: &mut Commands, material: Handle<ColorMaterial>, position: Position) {
        commands
            .spawn(SpriteComponents {
                material,
                ..Default::default()
            })
            .with(SnakeSegment { next_segment: None })
            .with(position)
            .with(Size::square(0.65));
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self: &Self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Bevy requires a specific ordering to the params when registering systems.
    // Commands → Resources → Components/Queries.
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(HeadMaterial(
        materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    ));
    commands.insert_resource(SegmentMaterial(
        materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    ));
}

fn game_setup(
    mut commands: Commands,
    head_material: Res<HeadMaterial>,
    segment_material: Res<SegmentMaterial>,
) {
    SnakeSegment::spawn_segment(&mut commands, segment_material.0, Position { x: 10, y: 9 });
    let first_segment = commands.current_entity().unwrap();
    commands
        .spawn(SpriteComponents {
            material: head_material.0,
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(SnakeHead {
            direction: Direction::Up,
            next_segment: first_segment,
        })
        .with(Position { x: 10, y: 10 })
        .with(Size::square(0.8));
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

fn snake_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_timer: ResMut<SnakeMoveTimer>,
    mut head_position: Query<(&mut SnakeHead, &mut Position)>,
) {
    snake_timer.0.tick(time.delta_seconds);
    for (mut head, mut head_pos) in &mut head_position.iter() {
        let mut dir: Direction = head.direction;
        if keyboard_input.pressed(KeyCode::Left) {
            dir = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            dir = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            dir = Direction::Down;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            dir = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Y) {
            head_pos.x = 0;
            head_pos.y = 0;
        }
        if keyboard_input.just_pressed(KeyCode::T) {
            eprintln!("Pos X: {}, Pos Y: {}", head_pos.x, head_pos.y);
        }

        //Y wrap around
        //if pos.y > 40 {
        //pos.y = 0;
        //} else if pos.y < 0 {
        //pos.y = 40;
        //}

        //X wrap around
        //if pos.x > 40 {
        //pos.x = 0;
        //} else if pos.x < 0 {
        //pos.x = 40;
        //}
        if dir != head.direction.opposite() {
            head.direction = dir;
        }

        if snake_timer.0.finished {
            match &head.direction {
                Direction::Left => {
                    head_pos.x -= 1;
                }
                Direction::Right => {
                    head_pos.x += 1;
                }
                Direction::Up => {
                    head_pos.y += 1;
                }
                Direction::Down => {
                    head_pos.y -= 1;
                }
            }
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
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup") // Not quite sure what Stage is doing here but lets keep going.
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_default_plugins()
        .run();
}
