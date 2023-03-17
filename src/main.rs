use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::*;

mod input;
use input::*;

/// Height of the window.
pub const HEIGHT: f32 = 720.0;
/// Width of the window.
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH, HEIGHT).into(),
                        title: "Bevy Minesweeper".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(BoardPlugin)
        .add_plugin(MyInputPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_textures)
        .run();
}

/// A tag to keep track of the main camera.
#[derive(Component)]
struct MainCamera;

/// Adds the camera to the world and tags it as the main one.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Holds the textures for the tiles, laid out as follows:
/// - 0-8: An empty cell with that many neighbors
/// - 9: A hidden cell
/// - 10: A flagged cell
/// - 11: A bomb
#[derive(Resource, Deref, DerefMut)]
pub struct TileTextures(Handle<TextureAtlas>);

fn load_textures(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        assets.load("tiles.png"),
        Vec2::new(16.0, 16.0),
        1,
        12,
        None,
        None,
    );

    commands.insert_resource(TileTextures(texture_atlases.add(texture_atlas)));
}
