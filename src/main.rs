use std::f32::consts::PI;

use bevy::{prelude::*, time::FixedTimestep, transform};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_racket),
        )
        .run();
}

#[derive(Component, Debug)]
struct Player {
    player_number: i32,
    movement_keys: MovementKeys,
}

#[derive(Debug)]
struct MovementKeys {
    up: KeyCode,
    down: KeyCode,
}

struct Game {
    score_to_win: i32,
}

#[derive(Component)]
struct Racket {
    player_number: i32,
}

#[derive(Component)]
struct Ball;

const TIME_STEP: f32 = 1.0 / 60.0;
const RACKET_SPEED: f32 = 120.0;

const RACKET_THICCNESS: f32 = 40.0;
const RACKET_WALL_OFFSET: f32 = 20.0;
const RACKET_SIZE: Vec3 = Vec3::new(120.0, RACKET_THICCNESS, 0.0);
const RACKET_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

const WALL_THICKNESS: f32 = 30.0;

const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const TOP_WALL: f32 = 250.0;
const BOTTOM_WALL: f32 = -250.0;

const WALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    #[bundle]
    sprite_bundle: SpriteBundle,
    //collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            //collider: Collider,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let player1 = Player {
        player_number: 1,
        movement_keys: MovementKeys {
            up: KeyCode::W,
            down: KeyCode::S,
        },
    };

    let player2 = Player {
        player_number: 2,
        movement_keys: MovementKeys {
            up: KeyCode::Up,
            down: KeyCode::Down,
        },
    };

    commands.spawn().insert(player1);
    commands.spawn().insert(player2);

    spawn_racket(&mut commands, true);
    spawn_racket(&mut commands, false);

    spawn_ball(&mut commands);

    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}

fn spawn_ball(commands: &mut Commands) {
    const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
    const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    const BALL_COLOR: Color = Color::rgb(0.9, 0.5, 0.0);

    commands.spawn().insert(Ball).insert_bundle(SpriteBundle {
        transform: Transform {
            scale: BALL_SIZE,
            translation: BALL_STARTING_POSITION,
            ..default()
        },
        sprite: Sprite {
            color: BALL_COLOR,
            ..default()
        },
        ..default()
    });
    //.insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
}

fn move_racket(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Racket, &mut Transform)>,
    player_query: Query<&Player>,
) {
    let players_who_moved: Vec<&Player> = player_query
        .iter()
        .filter(|player| {
            return keyboard_input.pressed(player.movement_keys.up)
                || keyboard_input.pressed(player.movement_keys.down);
        })
        .collect();

    if players_who_moved.len() < 1 {
        return;
    }

    for player in players_who_moved.into_iter() {
        let direction = if keyboard_input.pressed(player.movement_keys.up) {
            1.0
        } else {
            -1.0
        };

        for (racket, mut transform) in query.iter_mut().filter(|stuff| {
            return stuff.0.player_number == player.player_number;
        }) {
            let new_position = transform.translation.y + direction * RACKET_SPEED * TIME_STEP;
            transform.translation.y = new_position;
        }
    }
}

fn spawn_racket(commands: &mut Commands, player2: bool) {
    let racket_location = if player2 {
        Vec3::new(RIGHT_WALL - RACKET_THICCNESS - RACKET_WALL_OFFSET, 0.0, 0.0)
    } else {
        Vec3::new(LEFT_WALL + RACKET_THICCNESS + RACKET_WALL_OFFSET, 0.0, 0.0)
    };

    let player_number = if player2 == true { 2 } else { 1 };

    commands
        .spawn()
        .insert(Racket { player_number })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: racket_location,
                rotation: Quat::from_rotation_z(90.0 * PI / 180.0),
                scale: RACKET_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: RACKET_COLOR,
                ..default()
            },
            ..default()
        });
}
