use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{HEIGHT, WIDTH};

/// Struct that defines a bounding box.
#[derive(Reflect, Default)]
struct Bound {
    bottom_left: Vec2,
    top_right: Vec2,
}

impl Bound {
    /// Creates a new bound from the given bottom left (inclusive) and top right (exclusive) corners.
    fn new(bottom_left: Vec2, top_right: Vec2) -> Self {
        Self {
            bottom_left,
            top_right,
        }
    }

    /// Checks that a given location is inside the bounding box.
    fn in_bounds(&self, position: Vec2) -> bool {
        (self.bottom_left.x <= position.x)
            & (self.top_right.x > position.x)
            & (self.bottom_left.y <= position.y)
            & (self.top_right.y > position.y)
    }
}

/// Defines the different interactions that something can have with the cursor.
#[derive(Default, Reflect, Clone, Copy)]
enum InteractionType {
    #[default]
    /// No interaction.
    None,
    /// Cursor is on object but no mouse button is pressed.
    Hovered,
    /// First frame of the mouse button being pressed.
    Clicked,
    /// Any other frame of the mouse button being pressed.
    Held,
    /// First frame of the mouse button being released.
    Released,
}

/// Component added to any entity that can be selected.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Selectable {
    bound: Bound,
    interaction: InteractionType,
}

impl Selectable {
    /// Creates a new selectable object from the given bottom left (inclusive) and top right (exclusive) corners.
    pub fn new(bottom_left: Vec2, top_right: Vec2) -> Self {
        Self {
            bound: Bound::new(bottom_left, top_right),
            interaction: InteractionType::None,
        }
    }
}

/// Sets the selection types of any Selectable entities based on the cursor position and mouse button state.
fn set_selection(
    mut selectables: Query<&mut Selectable>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let Some(position) = window.cursor_position() else { return };
    let position = position - Vec2::new(WIDTH/2., HEIGHT/2.);

    let interaction;

    if mouse_input.just_pressed(MouseButton::Left) {
        interaction = InteractionType::Clicked;
    } else if mouse_input.pressed(MouseButton::Left) {
        interaction = InteractionType::Held;
    } else if mouse_input.just_released(MouseButton::Left) {
        interaction = InteractionType::Released;
    } else {
        interaction = InteractionType::Hovered;
    }

    for mut selectable in &mut selectables {
        if selectable.bound.in_bounds(position) {
            selectable.interaction = interaction;
        } else {
            selectable.interaction = InteractionType::None;
        }
    }
}

/// Bundles the code in this module to be used in the main app.
pub struct MyInputPlugin;

impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selectable>()
            .add_system(set_selection);
    }
}
