use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const FULL_ROTATION: f32 = 2.0 * std::f32::consts::PI;

#[derive(Component, Inspectable, Debug)]
pub struct Player {
    /// The current gravity affecting the player.
    pub gravity: Gravity,

    /// When the gravity was last changed.
    pub gravity_change_time: f64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            gravity: Gravity::default(),
            gravity_change_time: 0.0,
        }
    }
}

#[derive(Inspectable, Debug)]
pub enum Gravity {
    Up,
    Down,
    Left,
    Right,
}

impl Gravity {
    pub fn rotation(&self) -> f32 {
        use Gravity::*;

        match self {
            Down=> 0.0,
            Right => FULL_ROTATION * 0.25,
            Up => FULL_ROTATION * 0.5,
            Left => FULL_ROTATION * 0.75,
        }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity::Down
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/player/player.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0).with_scale(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        })
        .insert(Player::default())
        .insert(Name::new("Player"));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player in query.iter() {
        // If the player ever gets children, make recursive.
        commands.entity(player).despawn();
    }
}

pub fn update_player_input(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 100.0 * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::I) {
        player.gravity = Gravity::Up;
        player.gravity_change_time = time.seconds_since_startup();
    }

    if keyboard.pressed(KeyCode::K) {
        player.gravity = Gravity::Down;
        player.gravity_change_time = time.seconds_since_startup();
    }

    if keyboard.pressed(KeyCode::J) {
        player.gravity = Gravity::Left;
        player.gravity_change_time = time.seconds_since_startup();
    }

    if keyboard.pressed(KeyCode::L) {
        player.gravity = Gravity::Right;
        player.gravity_change_time = time.seconds_since_startup();
    }
}

pub fn rotate_player(mut query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
    let (player, mut transform) = query.single_mut();

    // Spinnnn
    // transform.rotate(Quat::from_rotation_z(1.0 * time.delta_seconds()));

    // Rotate to given gravity
    //transform.rotation = Quat::from_rotation_z(player.gravity.rotation());

    //transform.rotation = transform.rotation.lerp(Quat::from_rotation_z(player.gravity.rotation()), (time.seconds_since_startup() - player.gravity_change_time) as f32);

    let time_diff = (time.seconds_since_startup() - player.gravity_change_time).min(1.0) as f32;

    transform.rotation = Quat::from_rotation_z(transform.rotation.z.abs() + time_diff * (player.gravity.rotation() - transform.rotation.z.abs()));
}
