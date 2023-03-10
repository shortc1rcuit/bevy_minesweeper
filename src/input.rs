use crate::MainCamera;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

/// Resource that contains the location of the cursor in world space.
#[derive(Reflect, Resource, Deref, DerefMut, Default)]
#[reflect(Resource)]
struct CursorWorldPos(Option<Vec2>);

//Taken from https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games
/// Converts the location of the cursor on the screen to the location in the world.
fn cursor_to_world_pos(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_world_pos: ResMut<CursorWorldPos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        cursor_world_pos.0 = Some(world_pos);
    } else {
        cursor_world_pos.0 = None;
    }
}

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
    cursor_pos: Res<CursorWorldPos>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let Some(position) = cursor_pos.0 else { return };

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
        app.init_resource::<CursorWorldPos>()
            .register_type::<CursorWorldPos>()
            .register_type::<Selectable>()
            .add_system(cursor_to_world_pos)
            .add_system(set_selection.after(cursor_to_world_pos));
    }
}
