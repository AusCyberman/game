use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform,
    window::{close_on_esc, PrimaryWindow},
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (move_circle, rotate_gun, spawn_bullet, move_bullet).chain(),
        )
        .add_systems(Update, close_on_esc)
        .run();
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity(pub Vec2);

fn move_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Bullet>>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0);
        if transform.translation.x > X_EXTENT
            || transform.translation.x < -X_EXTENT
            || transform.translation.y > Y_EXTENT
            || transform.translation.y < -Y_EXTENT
        {
            commands.entity(entity).despawn();
        }
    }
}

fn rotate_gun(
    q_parent: Query<&GlobalTransform, With<Player>>,
    mut query: Query<(&Parent, &mut Transform), With<Gun>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = q_windows.single();
    let (camera, camera_transform) = q_camera.single();
    let cursor_position = if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|cursor| cursor.origin.truncate())
    {
        position
    } else {
        return;
    };
    for (parent, mut transform) in query.iter_mut() {
        // point towards cursor, where parent is cursor
        // maintain 30 units from parent
        let parent_transform = q_parent.get(parent.get()).unwrap();

        let direction = cursor_position - parent_transform.translation().xy();
        let angle = direction.y.atan2(direction.x);
        transform.rotation = Quat::from_rotation_z(angle);

        transform.translation = direction.normalize().extend(1.0) * 30.0;
    }
}

fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    gun_transform: Query<(&GlobalTransform, &Transform), With<Gun>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    // point towards cursor
    let (global_gun, relative_gun) = gun_transform.single();
    let vel = relative_gun.translation.truncate().normalize() * 5.0;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box {
                    max_x: 10.0,
                    max_y: 5.0,
                    min_x: -10.0,
                    min_y: -5.0,

                    ..Default::default()
                })
                .into(),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: global_gun.compute_transform(),
            ..Default::default()
        },
        Velocity(vel),
        Bullet,
    ));
}

fn move_circle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.y += 10.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.y -= 10.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= 10.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += 10.0;
    }

    transform.translation.x = transform.translation.x.clamp(-X_EXTENT, X_EXTENT);

    transform.translation.y = transform.translation.y.clamp(-Y_EXTENT, Y_EXTENT);
}
const X_EXTENT: f32 = 600.;

const Y_EXTENT: f32 = 300.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // Circle player
    let player = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: 30.0,
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(Color::rgb(1.0, 0.0, 0.5)),
                transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                ..Default::default()
            },
            Player,
        ))
        .id();
    // gun
    let gun = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Box {
                        min_x: -5.0,
                        min_y: -5.0,
                        max_x: 5.0,
                        max_y: 5.0,
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(Color::rgb(0.0, 1.0, 0.5)),
                transform: Transform::from_translation(Vec3::new(-5.0, 30.0, 1.0)),
                ..Default::default()
            },
            Gun,
        ))
        .id();
    commands.entity(player).push_children(&[gun]);
}
