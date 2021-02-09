#![allow(clippy::many_single_char_names)]
#![feature(iterator_fold_self)]
use bevy::render::{mesh::Indices, pipeline::PrimitiveTopology};
use bevy::{prelude::*, window::WindowMode};
use rand::{thread_rng, Rng};

use break_in::{color, window};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: TITLE.into(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<window::PrevWindow>()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Score(0))
        .add_resource(DilatedTime::new(BASE_DILATION))
        .add_resource(Paused(false)) // temporary
        .add_event::<Collision>()
        .add_startup_system_to_stage(bevy::app::startup_stage::PRE_STARTUP, setup.system())
        .add_startup_system(start.system()) // temporary
        .add_system(window::track_window.system())
        .add_system(dilate_time.system())
        .add_system(copy_transforms.system())
        .add_system(toggle_paused.system())
        .add_system(toggle_screen.system())
        .add_system(change_dilation.system())
        .add_system(input.system())
        .add_system(paddle_movement.system())
        .add_system(ball_movement.system())
        .add_system(collisions.system())
        .add_system(handle_collisions.system())
        .add_system(window::compare_window.system())
        .run();
}

// Config constants

const BALL_RADIUS: f32 = 10.;
const BALL_VELOCITY: [f32; 2] = [100.; 2];
/// Space in the centre to leave empty
const BRICK_EMPTY_RING_RADIUS: f32 = 15.;
/// Minimum gap between bricks within the same ring
const BRICK_MIN_CIRCUMFERENTIAL_GAP: f32 = 10.;
/// Gap between rings
const BRICK_RING_GAP: f32 = 10.;
/// Number of rings
const BRICK_RINGS: usize = 5;
// const BRICK_RINGS: usize = 1;
const BRICK_THICKNESS: f32 = 20.;
const BRICK_WIDTH: f32 = 40.;
const PADDLE_RING_RADIUS: f32 = 320.;
const PADDLE_THICKNESS: f32 = 20.;
const PADDLE_WIDTH: f32 = 80.;
const PADDLE_SPEED: f32 = PI;
const HITS_LEFT_VS_COLOR: [Color; 5] = [
    color::CADET_BLUE,
    color::SPRING_GREEN,
    color::YELLOW,
    color::ORANGE,
    color::RED,
];
const TITLE: &str = "Break-in";

// Helper constants

use std::{collections::HashMap, f32::consts::PI, usize};
const TWO_PI: f32 = 2. * PI;
const HALF_PI: f32 = PI / 2.;
const BASE_DILATION: f32 = 0.5;

// Components

#[derive(Debug)]
struct Paddle {
    destination_coords: Vec2,
    destination_location: DestinationLocation,
    rotational_speed: f32,
}

#[derive(Debug)]
struct Brick {
    hits_left: usize,
    points_awarded: usize,
}

struct Ball {
    velocity: Vec2,
}

struct Despawnable;

// Sub types

#[derive(Debug, Clone, Copy)]
enum DestinationLocation {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

// Resources (State)

struct Score(usize);

#[derive(Default)]
struct BrickColors(Vec<Handle<ColorMaterial>>);

// circle to circle: radius + radius vs distance between origins
// circle to rectangle: radius vs distance for each side
struct Collider {
    radius: f32,
}

struct DilatedTime {
    dt: f32,
    dilation: f32,
}

impl DilatedTime {
    fn new(dilation: f32) -> Self {
        Self { dt: 0., dilation }
    }

    fn delta_seconds(&self) -> f32 {
        self.dt
    }
}

// temporary
struct Paused(bool);

// Events

#[derive(Debug)]
struct Collision {
    from: Entity,
    to: Entity,
    /// Sides and distances
    sides: Vec<([Vec2; 2], f32)>,
}

// Setup systems

fn setup(
    commands: &mut Commands,
    assets: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(BrickColors(
        HITS_LEFT_VS_COLOR
            .iter()
            .map(|&color| materials.add(color.into()))
            .collect(),
    ));

    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".into(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: color::YELLOW,
                        },
                    },
                    TextSection {
                        value: "0".into(),
                        style: TextStyle {
                            font: assets.load("fonts/JetBrainsMono-ExtraBold.ttf"),
                            font_size: 40.,
                            color: color::ALICE_BLUE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
}

// Game start systems

fn start(
    commands: &mut Commands,
    colors: Res<BrickColors>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: ResMut<AssetServer>,
) {
    let mut rng = thread_rng();
    let brick_radius = (BRICK_WIDTH * BRICK_WIDTH + BRICK_THICKNESS * BRICK_THICKNESS).sqrt() / 2.;
    for ring in 0..BRICK_RINGS {
        // first, calculate the number of bricks
        let radius = BRICK_EMPTY_RING_RADIUS
            + BRICK_THICKNESS / 2.
            + ring as f32 * (BRICK_RING_GAP + BRICK_THICKNESS);
        let circumference = TWO_PI * radius;
        let brick_count = circumference / (BRICK_WIDTH + BRICK_MIN_CIRCUMFERENTIAL_GAP);
        let brick_count = brick_count as usize;
        for brick in 0..brick_count {
            let hits_left = rng.gen_range(0..5);
            let points_awarded = hits_left;
            let angle = brick as f32 / brick_count as f32 * TWO_PI;
            let arbitrary_offset = ring as f32 * BRICK_RING_GAP;
            let angle = angle + arbitrary_offset;
            let translation = xy_from_angle(angle, radius).extend(0.);
            // kinda cool looking, but useless, saved for something else
            // let rotation = Quat::from_axis_angle(Vec3::new(x, y, 1.0), arc - HALF_PI);
            let rotation = Quat::from_rotation_z(angle - HALF_PI);
            // let rotation = Quat::default();
            let entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        size: Vec2::new(BRICK_WIDTH, BRICK_THICKNESS),
                        ..Default::default()
                    },
                    material: colors.0[hits_left].clone_weak(),
                    transform: Transform {
                        translation,
                        rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    hits_left,
                    points_awarded,
                })
                .with(Collider {
                    radius: brick_radius,
                })
                .with(Despawnable)
                .current_entity()
                .unwrap();
            let text = commands
                .spawn(Text2dBundle {
                    text: Text::with_section(
                        // format!("{:?}", entity.id()),
                        format!("{}", hits_left),
                        TextStyle {
                            font: assets.load("fonts/JetBrainsMono-ExtraBold.ttf"),
                            font_size: 15.0,
                            ..Default::default()
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();
            commands.push_children(entity, &[text]);
        }
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(2. * BALL_RADIUS, 2. * BALL_RADIUS),
                ..Default::default()
            },
            material: materials.add(color::AQUAMARINE.into()),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            mesh: meshes.add(polygon(0.5, 360)),
            ..Default::default()
        })
        .with(Ball {
            velocity: Vec2::from(BALL_VELOCITY),
        })
        .with(Collider {
            radius: BALL_RADIUS,
        })
        .with(PastTransform::default())
        .with(Despawnable);

    let paddle = Paddle {
        destination_coords: Vec2::new(0., -PADDLE_RING_RADIUS),
        destination_location: DestinationLocation::Bottom,
        rotational_speed: PADDLE_SPEED, // two seconds to make a full revolution
    };
    let paddle_radius =
        (PADDLE_WIDTH * PADDLE_WIDTH + PADDLE_THICKNESS * PADDLE_THICKNESS).sqrt() / 2.;
    let translation = Vec3::new(PADDLE_RING_RADIUS, 0., 0.);
    let rotation = Quat::from_rotation_z(angle_from_xy(translation.truncate()) - HALF_PI);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(PADDLE_WIDTH, PADDLE_THICKNESS),
                ..Default::default()
            },
            material: materials.add(color::GREY.into()),
            transform: Transform {
                translation,
                rotation,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(paddle)
        .with(Collider {
            radius: paddle_radius,
        })
        .with(Despawnable);
}

// Play systems (physics)

fn dilate_time(time: Res<Time>, mut dilated_time: ResMut<DilatedTime>) {
    let dt = time.delta_seconds() * dilated_time.dilation;
    dilated_time.dt = dt;
}

/// At the start of the tick, copy the previous positions / rotations.
/// This will be used:
/// - to allow backtracking
/// - and later, to interpolate between past and future positions for smooth rendering.
fn copy_transforms(
    // TODO: look into optimising with Changed<T>
    // query: Query<(&mut PastTransform, &FutureTransform)>,
    mut query: Query<(&mut PastTransform, &Transform)>,
) {
    for (mut past, future) in query.iter_mut() {
        past.0 = *future;
    }
}

fn ball_movement(
    paused: Res<Paused>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    time: Res<DilatedTime>,
) {
    let dt = time.delta_seconds();
    if paused.0 {
        return;
    }
    for (mut transform, mut ball) in ball_query.iter_mut() {
        let new_translation = transform.translation.truncate() + ball.velocity * dt;
        // temporary
        if new_translation.length_squared() >= square(PADDLE_RING_RADIUS - PADDLE_THICKNESS) {
            // ball.velocity = -ball.velocity;
            let n = transform.translation.truncate().normalize();
            let dot = ball.velocity.dot(n);
            // https://gamedev.stackexchange.com/questions/112299/balls-velocity-vector-reflect-against-a-point
            // r=d−2(d⋅n)n
            ball.velocity -= 2. * dot * n;
        }
        transform.translation = new_translation.extend(0.);
    }
}

fn paddle_movement(
    paused: Res<Paused>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
    time: Res<DilatedTime>,
) {
    if paused.0 {
        return;
    }
    // the paddle moves along the shortest circular path to its destination
    let dt = time.delta_seconds();
    for (mut transform, paddle) in paddle_query.iter_mut() {
        let current_angle = angle_from_xy(transform.translation.truncate());
        let destination_angle = angle_from_xy(paddle.destination_coords);
        let relative_angle = wrap_negpi_pi(destination_angle - current_angle);
        let delta_angle = paddle.rotational_speed * dt;
        if delta_angle.abs() >= relative_angle.abs() {
            transform.translation = paddle.destination_coords.extend(0.);
        } else {
            let delta_angle = relative_angle.signum() * delta_angle;
            transform.translation =
                xy_from_angle(current_angle + delta_angle, PADDLE_RING_RADIUS).extend(0.);
        }
        transform.rotation =
            Quat::from_rotation_z(angle_from_xy(transform.translation.truncate()) - HALF_PI);
    }
}

#[allow(clippy::type_complexity)]
/// this is the hard bit
fn collisions(
    paused: Res<Paused>,
    ball_query: Query<(Entity, &Transform, &Collider), With<Ball>>,
    collider_query: Query<
        (Entity, &Transform, &Collider, &Sprite),
        Or<(With<Paddle>, With<Brick>)>,
    >,
    mut collisions: ResMut<Events<Collision>>,
) {
    if paused.0 {
        return;
    }
    for (from, ball_transform, ball_collider) in ball_query.iter() {
        let ball_centre = ball_transform.translation.truncate();
        for (to, transform, collider, sprite) in collider_query.iter() {
            // first check if they are even in range, should be reasonably cheap
            if in_range(
                ball_transform.translation.truncate(),
                ball_collider.radius,
                transform.translation.truncate(),
                collider.radius,
            ) {
                let sides = RectangleSideIterator::new(transform, sprite.size)
                    .map(|side| {
                        let distance = point_to_line_distance(ball_centre, side);
                        (side, distance)
                    })
                    .filter(|(_side, distance)| distance <= &ball_collider.radius)
                    .collect::<Vec<_>>();
                if !sides.is_empty() {
                    // this is not a complete solution
                    // see: https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
                    // while points are captured, we still need to consider the case of a circle within the
                    // however, we should always be approaching from the outside (and backtracking) so it should be OK
                    collisions.send(Collision { from, to, sides });
                }
            }
        }
    }
}

struct CollisionPoint;

#[allow(clippy::too_many_arguments)]
fn handle_collisions(
    // mut paused: ResMut<Paused>,
    _time: Res<DilatedTime>,
    mut collisions: EventReader<Collision>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_points: Query<Entity, With<CollisionPoint>>,
    mut despawned_already: Local<bool>,
    commands: &mut Commands,
    mut ball_query: Query<(Entity, &mut Ball, &mut Transform, &PastTransform)>,
) {
    let mut rng = thread_rng();
    let mut map: HashMap<Entity, ([Vec2; 2], f32, Entity)> = HashMap::new();
    for collision in collisions.iter() {
        for (entity, _ball, _transform, _past_transform) in ball_query.iter_mut() {
            if entity == collision.from {
                for (side, _) in collision.sides.iter() {
                    let [a, b] = side;

                    let red: f32 = rng.gen_range(0.5..=1.0);
                    let green: f32 = rng.gen_range(0.5..=1.0);
                    let blue: f32 = rng.gen_range(0.5..=1.0);
                    let color = materials.add(Color::rgba(red, green, blue, 0.9).into());

                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                size: Vec2::splat(6.),
                                ..Default::default()
                            },
                            material: color.clone(),
                            transform: Transform::from_translation(a.extend(20.)),
                            ..Default::default()
                        })
                        .with(CollisionPoint);
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                size: Vec2::splat(6.),
                                ..Default::default()
                            },
                            material: color,
                            transform: Transform::from_translation(b.extend(20.)),
                            ..Default::default()
                        })
                        .with(CollisionPoint);
                }

                if let Some((closest_side, distance)) =
                    collision.sides.iter().fold_first(|closest_side, side| {
                        if side.1 < closest_side.1 {
                            side
                        } else {
                            closest_side
                        }
                    })
                {
                    map.entry(collision.from)
                        .and_modify(|current_closest_side| {
                            if distance < &current_closest_side.1 {
                                *current_closest_side = (*closest_side, *distance, collision.to);
                            }
                        })
                        .or_insert((*closest_side, *distance, collision.to));
                };
            }
        }

        // paused.0 = true;
        if *despawned_already {
            *despawned_already = false;
        }
    }

    for (ball_entity, ([a, b], _distance, _collided_entity)) in map.drain() {
        match ball_query.get_mut(ball_entity) {
            Ok((_entity, mut ball, mut transform, past_transform)) => {
                // first, backtrack 1 tick
                *transform = past_transform.0;
                // now update the velocity
                // https://gamedev.stackexchange.com/questions/136073/how-does-one-calculate-the-surface-normal-in-2d-collisions
                // https://stackoverflow.com/questions/16417891/how-can-i-find-a-normal-vector-of-a-2d-line
                // dx=x2-x1 and dy=y2-y1, then the normals are (-dy, dx) and (dy, -dx)
                let d = b - a;
                let n1 = Vec2::new(-d.y, d.x).normalize();
                let n2 = -n1;
                let dot1 = ball.velocity.dot(n1);
                let dot2 = ball.velocity.dot(n2);
                let (n, dot) = if dot1 < dot2 { (n1, dot1) } else { (n2, dot2) };
                // https://gamedev.stackexchange.com/questions/112299/balls-velocity-vector-reflect-against-a-point
                // r=d−2(d⋅n)n
                ball.velocity -= 2. * dot * n;
            }
            Err(err) => {
                println!("query error: {:?}", err);
            }
        }
    }

    if !*despawned_already {
        for entity in existing_points.iter() {
            commands.despawn(entity);
        }
        *despawned_already = true;
    }
}

// Play systems (other)

fn input(paused: Res<Paused>, input: Res<Input<KeyCode>>, mut paddle_query: Query<&mut Paddle>) {
    if paused.0 {
        return;
    }
    let mut x = 0;
    let mut y = 0;
    if input.just_pressed(KeyCode::Right)
        || input.just_pressed(KeyCode::Left)
        || input.just_pressed(KeyCode::Up)
        || input.just_pressed(KeyCode::Down)
    {
        if input.pressed(KeyCode::Right) {
            x += 1;
        }
        if input.pressed(KeyCode::Left) {
            x -= 1;
        }
        if input.pressed(KeyCode::Up) {
            y += 1;
        }
        if input.pressed(KeyCode::Down) {
            y -= 1;
        }
    } else {
        return;
    }
    for mut paddle in paddle_query.iter_mut() {
        paddle.destination_location = match (x, y) {
            (0, 1) => DestinationLocation::Top,
            (1, 1) => DestinationLocation::TopRight,
            (1, 0) => DestinationLocation::Right,
            (1, -1) => DestinationLocation::BottomRight,
            (0, -1) => DestinationLocation::Bottom,
            (-1, -1) => DestinationLocation::BottomLeft,
            (-1, 0) => DestinationLocation::Left,
            (-1, 1) => DestinationLocation::TopLeft,
            _ => paddle.destination_location,
        };
        paddle.destination_coords = match paddle.destination_location {
            DestinationLocation::Top => Vec2::new(0., 1.).normalize(),
            DestinationLocation::TopRight => Vec2::new(1., 1.).normalize(),
            DestinationLocation::Right => Vec2::new(1., 0.).normalize(),
            DestinationLocation::BottomRight => Vec2::new(1., -1.).normalize(),
            DestinationLocation::Bottom => Vec2::new(0., -1.).normalize(),
            DestinationLocation::BottomLeft => Vec2::new(-1., -1.).normalize(),
            DestinationLocation::Left => Vec2::new(-1., 0.).normalize(),
            DestinationLocation::TopLeft => Vec2::new(-1., 1.).normalize(),
        } * PADDLE_RING_RADIUS;
        angle_from_xy(paddle.destination_coords);
    }
}

// debug
fn toggle_paused(input: Res<Input<KeyCode>>, mut paused: ResMut<Paused>) {
    if input.just_pressed(KeyCode::Space) {
        paused.0 = true;
    }
    if input.just_released(KeyCode::Space) {
        paused.0 = false;
    }
}

// debug
fn toggle_screen(input: Res<Input<KeyCode>>, mut window_descriptor: ResMut<WindowDescriptor>) {
    if input.just_pressed(KeyCode::Escape) {
        window_descriptor.mode = match window_descriptor.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            _ => unreachable!(),
        }
    }
}

// debug
fn change_dilation(input: Res<Input<KeyCode>>, mut time: ResMut<DilatedTime>) {
    if input.just_pressed(KeyCode::NumpadAdd) {
        time.dilation *= 2.0;
    }
    if input.just_pressed(KeyCode::NumpadSubtract) {
        time.dilation *= 0.5;
    }
}

// Game over systems

fn _end() {
    // despawn entities
    // reset score
}

// Utilities

// consider r-star: https://docs.rs/rstar/0.8.2/rstar/struct.RTree.html
fn in_range(position_a: Vec2, radius_a: f32, position_b: Vec2, radius_b: f32) -> bool {
    let vector: Vec2 = position_a - position_b;
    vector.length_squared() <= square(radius_a + radius_b)
}

/// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
fn point_to_line_distance(point: Vec2, [line_start, line_end]: [Vec2; 2]) -> f32 {
    let (x0, y0) = point.into();
    let (x1, y1) = line_start.into();
    let (x2, y2) = line_end.into();
    let denominator = (line_end - line_start).length();
    let numerator = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).abs();
    numerator / denominator
}

struct RectangleSideIterator {
    centre: Vec2,
    points: [Vec2; 4],
    index: usize,
}

impl RectangleSideIterator {
    fn new(transform: &Transform, size: Vec2) -> Self {
        let rotation = transform.rotation;
        let (_, rotation) = rotation.to_axis_angle();

        let a = point_rotated(Vec2::new(size.x / 2., size.y / 2.), rotation);
        let b = point_rotated(Vec2::new(size.x / 2., -size.y / 2.), rotation);
        let c = point_rotated(Vec2::new(-size.x / 2., -size.y / 2.), rotation);
        let d = point_rotated(Vec2::new(-size.x / 2., size.y / 2.), rotation);

        Self {
            centre: transform.translation.truncate(),
            points: [a, b, c, d],
            index: 0,
        }
    }
}

impl Iterator for RectangleSideIterator {
    type Item = [Vec2; 2];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some([self.centre + self.points[0], self.centre + self.points[1]]),
            2 => Some([self.centre + self.points[1], self.centre + self.points[2]]),
            3 => Some([self.centre + self.points[2], self.centre + self.points[3]]),
            4 => Some([self.centre + self.points[3], self.centre + self.points[0]]),
            _ => None,
        }
    }
}

fn square(number: f32) -> f32 {
    number * number
}

/// https://gamedev.stackexchange.com/questions/86755/how-to-calculate-corner-positions-marks-of-a-rotated-tilted-rectangle
/// - `rotation` is the angle of rotation
/// - `corner` is relative to the centre of the rectangle
fn point_rotated(corner: Vec2, rotation: f32) -> Vec2 {
    let rc = rotation.cos();
    let rs = rotation.sin();
    Vec2::new(corner.x * rc - corner.y * rs, corner.x * rs + corner.y * rc)
}

fn angle_from_xy(translation: Vec2) -> f32 {
    if translation.y.abs() < std::f32::EPSILON {
        // if on the x axis
        if translation.x.is_sign_positive() {
            // if on the right
            0.
        } else {
            // if on the left
            // prefer clockwise, i.e. the half-open range [-PI, -PI)
            -PI
        }
    } else if translation.x.abs() < std::f32::EPSILON {
        // if on the y axis
        if translation.y.is_sign_positive() {
            // if on the top
            HALF_PI
        } else {
            // if on the bottom
            -HALF_PI
        }
    } else {
        let angle = (translation.y / translation.x).atan();
        if translation.x < 0. {
            if translation.y < 0. {
                angle - PI
            } else {
                angle + PI
            }
        } else {
            angle
        }
    }
}

fn xy_from_angle(angle: f32, radius: f32) -> Vec2 {
    let x = angle.cos() * radius;
    let y = angle.sin() * radius;
    Vec2::new(x, y)
}

/// Keep the number within the half-open range [-PI, PI)
fn wrap_negpi_pi(num: f32) -> f32 {
    wrap(num, -PI, PI)
}

/// Keep the number within the half-open range [0, TWO_PI)
fn _wrap_zero_twopi(num: f32) -> f32 {
    wrap(num, 0., TWO_PI)
}

/// Keep the number within the half-open range [-lower, upper)
fn wrap(mut num: f32, lower: f32, upper: f32) -> f32 {
    let range = upper - lower;
    loop {
        if num >= upper {
            num -= range;
        } else if num < lower {
            num += range;
        } else {
            break;
        }
    }
    num
}

fn polygon(radius: f64, sides: usize) -> Mesh {
    const NORMAL: [f32; 3] = [0.0, 0.0, 1.0];

    let step = 2. * std::f64::consts::PI / sides as f64;

    let mut positions = Vec::with_capacity(sides + 1);
    positions.push([0.0f32, 0., 0.]); // the centre

    let mut uvs = Vec::with_capacity(sides + 1);
    uvs.push([0.5f32, 0.5]); // the centre

    let mut normals = Vec::new();
    normals.resize(sides + 1, NORMAL);

    let mut indices = Vec::with_capacity(3 * sides);

    for index in 0..sides {
        let angle = index as f64 * step;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let u = (x + 1.) / 2.;
        let v = (1. - y) / 2.;
        let position = [x as f32, y as f32, 0.];
        let uv = [u as f32, v as f32];
        let [a, b, c] = if index + 1 == sides {
            [0, 1, index + 1] // index 0 being the centre
        } else {
            [0, index + 2, index + 1] // index 0 being the centre
        };
        positions.push(position);
        uvs.push(uv);
        indices.push(a as u32);
        indices.push(b as u32);
        indices.push(c as u32);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

// Rendering systems

#[derive(Default)]
struct PastTransform(Transform);

#[derive(Default)]
struct FutureTransform(Transform);

#[test]
fn construct_triangles() {
    let triangle = polygon(1.0, 3);
    dbg!(triangle);
    let diamond = polygon(1.0, 4);
    dbg!(diamond);
    let pentagon = polygon(1.0, 5);
    dbg!(pentagon);
    let hexagon = polygon(1.0, 6);
    dbg!(hexagon);
}
