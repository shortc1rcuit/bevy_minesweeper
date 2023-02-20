use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::*;

mod input;
use input::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: WIDTH,
                        height: HEIGHT,
                        title: "Bevy Minesweeper".to_string(),
                        resizable: true,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MyInputPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_textures)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

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
