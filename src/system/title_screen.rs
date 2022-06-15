use bevy::prelude::*;
use crate::*;

pub fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: palette::PINK.2.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Button",
                TextStyle {
                    font: asset_server.load("font/firasans_bold.ttf"),
                    font_size: 40.0,
                    color: palette::MONO.4,
                },
                Default::default(),
            ),
            ..default()
        });
    }).insert(entity_marker::TitleUIElement);
}

pub fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<entity_marker::TitleUIElement>>) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn_recursive();
    }
}
