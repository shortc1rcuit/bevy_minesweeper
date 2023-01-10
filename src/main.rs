use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .register_type::<TextureAtlasSprite>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: WIDTH,
                        height: HEIGHT,
                        title: "Bevy Minesweeper".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_textures)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_scene)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct TileTextures {
    tile_map: Handle<TextureAtlas>,
}

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

    commands.insert_resource(TileTextures {
        tile_map: texture_atlases.add(texture_atlas),
    });
}

pub const TILES_WIDTH: i32 = 8;
pub const TILES_HEIGHT: i32 = 5;

fn setup_scene(mut commands: Commands, tile_map: Res<TileTextures>) {
    let min_x = (-80 * TILES_WIDTH) / 2;
    let min_y = (-80 * TILES_HEIGHT) / 2;

    for x in 0..TILES_WIDTH {
        for y in 0..TILES_HEIGHT {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tile_map.tile_map.clone(),
                transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0)).with_translation(
                    Vec3::new(
                        (min_x + (80 * x) + 40) as f32,
                        (min_y + (80 * y) + 40) as f32,
                        0.0,
                    ),
                ),
                ..default()
            });
        }
    }
}
