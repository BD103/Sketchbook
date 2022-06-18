use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Debug)]
pub struct Player {
    gravity: Gravity,
    gravity_change: f64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            gravity: Gravity::default(),
            gravity_change: 0.0,
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
        match self {
            _ => 0.0,
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

pub fn update_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    // Spinnnnn
    transform.rotate(Quat::from_rotation_z(1.0 * time.delta_seconds()));

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
}
