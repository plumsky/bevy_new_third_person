use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>().add_systems(
        Update,
        (
            apply_interaction_palette,
            (trigger_on_press, btn_sounds).run_if(resource_exists::<AudioSources>),
        ),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: (Color, Color),
    pub hovered: (Color, Color),
    pub pressed: (Color, Color),
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (
            &Interaction,
            &InteractionPalette,
            &mut BorderColor,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut border_color, mut background) in &mut palette_query {
        let (bg, border) = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        };
        *background = bg.into();
        *border_color = border.into();
    }
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

// TODO: not sure it's possible to do efficiently with observers in 3d, like in BevyFlock,
// it's dropping FPS like crazy
fn btn_sounds(
    mut commands: Commands,
    settings: Res<Settings>,
    audio_sources: Res<AudioSources>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => audio_sources.btn_hover.clone(),
            Interaction::Pressed => audio_sources.btn_press.clone(),
            _ => continue,
        };
        commands.spawn(sfx(source, settings.sfx()));
    }
}
