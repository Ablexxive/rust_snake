use bevy::prelude::*;

use crate::common::{Direction, Paused, Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use crate::food::Food;
use crate::game_over::GameOverEvent;

// Snake Head Information
pub struct SnakeMoveTimer(pub Timer);
pub struct SnakeHead {
    pub direction: Direction,
    pub next_segment: Entity,
}
pub struct HeadMaterial(pub Handle<ColorMaterial>); // resource for storing snake head sprite

// Snake Tail Info
#[derive(Default)]
pub struct SnakeSegment {
    next_segment: Option<Entity>,
}
pub struct SegmentMaterial(pub Handle<ColorMaterial>);

impl SnakeSegment {
    pub fn spawn_segment(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        position: Position,
    ) {
        commands
            .spawn(SpriteComponents {
                material,
                ..Default::default()
            })
            .with(SnakeSegment { next_segment: None })
            .with(position)
            .with(Size::square(0.65));
    }
    pub fn spawn_initial_snake(
        mut commands: Commands,
        head_material: Handle<ColorMaterial>,
        segment_material: Handle<ColorMaterial>,
    ) {
        SnakeSegment::spawn_segment(&mut commands, segment_material, Position { x: 10, y: 9 });
        let first_segment = commands.current_entity().unwrap();
        commands
            .spawn(SpriteComponents {
                material: head_material,
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
}

pub fn snake_movement(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    segment_material: Res<SegmentMaterial>,
    paused: Res<Paused>,
    mut snake_timer: ResMut<SnakeMoveTimer>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
    mut head_position: Query<(&mut SnakeHead, &mut Position)>,
    segments: Query<&mut SnakeSegment>,
    positions: Query<&mut Position>,
    mut food_positions: Query<(Entity, &Food, &Position)>,
) {
    if paused.0 {
        return;
    }
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
        if dir != head.direction.opposite() {
            head.direction = dir;
        }

        ////Y wrap around
        //if head_pos.y > 40 {
        //head_pos.y = 0;
        //} else if head_pos.y < 0 {
        //head_pos.y = 40;
        //}

        ////X wrap around
        //if head_pos.x > 40 {
        //head_pos.x = 0;
        //} else if head_pos.x < 0 {
        //head_pos.x = 40;
        //}
        if snake_timer.0.finished {
            let mut last_position = *head_pos;
            let mut segment_entity = head.next_segment;
            loop {
                let segment = segments.get::<SnakeSegment>(segment_entity).unwrap();
                let mut segment_position = positions.get_mut::<Position>(segment_entity).unwrap();
                let current_position = *segment_position;
                *segment_position = last_position;
                last_position = current_position;
                if *head_pos == last_position {
                    game_over_events.send(GameOverEvent);
                }
                if let Some(next_segment_id) = segment.next_segment {
                    segment_entity = next_segment_id;
                } else {
                    break;
                }
            }
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
            for (ent, _food, food_pos) in &mut food_positions.iter() {
                if food_pos == &*head_pos {
                    SnakeSegment::spawn_segment(&mut commands, segment_material.0, last_position);
                    let new_segment = commands.current_entity();
                    let mut segment = segments.get_mut::<SnakeSegment>(segment_entity).unwrap();
                    segment.next_segment = new_segment;
                    commands.despawn(ent);
                }
            }
        }
        // Game over if you hit a wall.
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 > ARENA_WIDTH
            || head_pos.y as u32 > ARENA_HEIGHT
        {
            game_over_events.send(GameOverEvent);
        }
    }
}
