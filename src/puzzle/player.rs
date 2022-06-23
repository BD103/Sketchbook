use super::Gravity;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Debug)]
pub struct Player {
    /// The current gravity affecting the player.
    pub gravity: Gravity,

    /// The previous gravity type
    pub gravity_last: Gravity,

    /// When the gravity was last changed.
    pub gravity_change_time: f64,
}

impl Player {
    pub fn change_gravity(&mut self, gravity: Gravity, current_time: f64) {
        if self.gravity != gravity {
            self.gravity_last = self.gravity;
            self.gravity = gravity;
            self.gravity_change_time = current_time;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            gravity: Gravity::default(),
            gravity_last: Gravity::default(),
            gravity_change_time: 0.0,
        }
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

    // Move with WASD
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

    // Change gravity with IJKL
    if keyboard.pressed(KeyCode::I) {
        player.change_gravity(Gravity::Up, time.seconds_since_startup());
    }

    if keyboard.pressed(KeyCode::K) {
        player.change_gravity(Gravity::Down, time.seconds_since_startup());
    }

    if keyboard.pressed(KeyCode::J) {
        player.change_gravity(Gravity::Left, time.seconds_since_startup());
    }

    if keyboard.pressed(KeyCode::L) {
        player.change_gravity(Gravity::Right, time.seconds_since_startup());
    }
}

pub fn rotate_player(mut query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
    // let (player, mut transform) = query.single_mut();
}
