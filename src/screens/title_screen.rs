use crate::{entity_markers, palette, states::GameState};
use bevy::prelude::*;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(spawn_title_screen));
        app.add_system_set(SystemSet::on_exit(GameState::Title).with_system(despawn_title_screen));
        app.add_system_set(SystemSet::on_update(GameState::Title).with_system(update_title_screen));
    }
}

pub fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },

            color: palette::MONO.1.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("font/fira_sans_bold.ttf"),
                        font_size: 40.0,
                        color: palette::MONO.4,
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(entity_markers::TitleUIElement);
}

pub fn despawn_title_screen(
    mut commands: Commands,
    query: Query<Entity, With<entity_markers::TitleUIElement>>,
) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn_recursive();
    }
}

// TODO: Rewrite hacky code
pub fn update_title_screen(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = palette::MONO.4.into();
                text.sections[0].style.color = palette::MONO.0.into();
            }
            Interaction::Hovered => {
                *color = palette::MONO.2.into();
                text.sections[0].style.color = palette::MONO.4.into();
            }
            Interaction::None => {
                *color = palette::MONO.1.into();
                text.sections[0].style.color = palette::MONO.4.into();
            }
        }
    }
}
