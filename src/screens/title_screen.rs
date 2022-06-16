use crate::{palette, states::GameState};
use bevy::prelude::*;
use bevy::ecs::schedule::StateError;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(spawn_title_screen));
        app.add_system_set(SystemSet::on_exit(GameState::Title).with_system(despawn_title_screen));
        app.add_system_set(SystemSet::on_update(GameState::Title).with_system(update_title_screen));
    }
}

#[derive(Component)]
pub struct TitleScreenUI;

pub fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: palette::BLUE.2.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/fira_sans_bold.ttf"),
                        font_size: 40.0,
                        color: palette::MONO.4.into(),
                    },
                    Default::default(),
                ),
                ..default()
            });
        })
        .insert(TitleScreenUI)
        .insert(Name::new("Button"));
}

pub fn despawn_title_screen(mut commands: Commands, query: Query<Entity, With<TitleScreenUI>>) {
    for element in query.iter() {
        commands.entity(element).despawn_recursive();
    }
}

pub fn update_title_screen(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>, With<TitleScreenUI>),
    >,
    // Constrain to TitleScreenUI?
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections
                    .iter_mut()
                    .for_each(|section: &mut TextSection| {
                        section.style.color = palette::MONO.0.into()
                    });
                *color = palette::BLUE.4.into();

                match state.set(GameState::Level) {
                    Ok(()) | Err(StateError::AlreadyInState) | Err(StateError::StateAlreadyQueued) => {},
                    Err(_) => panic!("Error switching state to GameState::Level. File: '{}', Line: '{}'.", file!(), line!()),
                }
            }
            Interaction::Hovered => {
                text.sections
                    .iter_mut()
                    .for_each(|section: &mut TextSection| {
                        section.style.color = palette::MONO.4.into()
                    });
                *color = palette::BLUE.3.into();
            }
            Interaction::None => {
                text.sections
                    .iter_mut()
                    .for_each(|section: &mut TextSection| {
                        section.style.color = palette::MONO.4.into()
                    });
                *color = palette::BLUE.2.into();
            }
        }
    }
}
