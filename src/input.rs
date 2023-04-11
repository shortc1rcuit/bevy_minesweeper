use crate::board::board_gen::{TileState, TileType};
use crate::board::{GameBoard, TileEntity};
use crate::MainCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseWheel;

//Taken from https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games
/// Converts the location of the cursor on the screen to the location in the world.
fn cursor_to_world_pos(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    let Ok(wnd) = wnds.get_single() else {
        return None;
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

        Some(world_pos)
    } else {
        None
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

/// Component added to any entity that can be selected.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Selectable {
    bound: Bound,
}

impl Selectable {
    /// Creates a new selectable object from the given bottom left (inclusive) and top right (exclusive) corners.
    pub fn new(bottom_left: Vec2, top_right: Vec2) -> Self {
        Self {
            bound: Bound::new(bottom_left, top_right),
        }
    }
}

struct LeftClick {
    position: Vec2,
}

struct RightClick {
    position: Vec2,
}

struct ScrollVert {
    position: Vec2,
    factor: f32,
}

const SCROLL_FACTOR: f32 = 1.2;

fn mouse_handling(
    mouse_input: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    wnds: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut l_clicks: EventWriter<LeftClick>,
    mut r_clicks: EventWriter<RightClick>,
    mut v_scrolls: EventWriter<ScrollVert>,
) {
    let Some(position) = cursor_to_world_pos(wnds, q_camera) else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        l_clicks.send(LeftClick { position })
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        r_clicks.send(RightClick { position })
    }

    for scroll in scroll_evr.iter() {
        v_scrolls.send(ScrollVert { position, factor: SCROLL_FACTOR.powf(-scroll.y) })
    }
}

fn tile_reveal(
    mut tiles: Query<(&mut TextureAtlasSprite, &Selectable, &TileEntity)>,
    mut board: ResMut<GameBoard>,
    mut l_clicks: EventReader<LeftClick>,
) {
    for click in l_clicks.iter() {
        for (mut atlas, selection, tile) in &mut tiles {
            if selection.bound.in_bounds(click.position)
                && board.get_state(tile.x, tile.y) == TileState::Hidden
            {
                atlas.index = match board[(tile.x, tile.y)].get_type() {
                    TileType::Empty(x) => *x as usize,
                    TileType::Bomb => 11,
                };

                board.set_state(tile.x, tile.y, TileState::Visable);
            }
        }
    }
}

fn tile_flag(
    mut tiles: Query<(&mut TextureAtlasSprite, &Selectable, &TileEntity)>,
    mut board: ResMut<GameBoard>,
    mut r_clicks: EventReader<RightClick>,
) {
    for click in r_clicks.iter() {
        for (mut atlas, selection, tile) in &mut tiles {
            if selection.bound.in_bounds(click.position) {
                match board.get_state(tile.x, tile.y) {
                    TileState::Visable => {}
                    TileState::Flagged => {
                        atlas.index = 9;
                        board.set_state(tile.x, tile.y, TileState::Hidden)
                    }
                    TileState::Hidden => {
                        atlas.index = 10;
                        board.set_state(tile.x, tile.y, TileState::Flagged);
                    }
                }
            }
        }
    }
}

fn zoom(mut camera: Query<&mut Transform, With<MainCamera>>, mut v_scrolls: EventReader<ScrollVert>,) {
    let mut camera = camera.iter_mut().next().unwrap();

    for scroll in v_scrolls.iter() {
        camera.scale.x *= scroll.factor;
        camera.scale.y *= scroll.factor;
    }
}

/// Bundles the code in this module to be used in the main app.
pub struct MyInputPlugin;

impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selectable>()
            .add_event::<LeftClick>()
            .add_event::<RightClick>()
            .add_event::<ScrollVert>()
            .add_systems((
                mouse_handling,
                tile_reveal.after(mouse_handling),
                tile_flag.after(mouse_handling),
                zoom.after(mouse_handling)
            ));
    }
}
