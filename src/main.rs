use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContext},
    quick::WorldInspectorPlugin,
};
use rand::prelude::*;

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
        .add_system(generate_board)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

#[derive(Default)]
struct BoardSettings {
    width: i32,
    height: i32,
}

fn generate_board(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut board_settings: Local<BoardSettings>,
    mut tiles: Query<Entity, With<TextureAtlasSprite>>,
    tile_map: Res<TileTextures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    egui::Window::new("Generate board").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Board dimensions:");

        ui.horizontal(|ui| {
            ui.label("Width:");

            ui.add(
                egui::DragValue::new(&mut board_settings.width)
                    .speed(0.1)
                    .clamp_range(1..=16),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Height:");

            ui.add(
                egui::DragValue::new(&mut board_settings.height)
                    .speed(0.1)
                    .clamp_range(1..=9),
            );
        });

        if ui.button("Generate").clicked() {
            for entity in &mut tiles {
                commands.entity(entity).despawn_recursive();
            }

            let tile_count = texture_atlases
                .get(&tile_map.0)
                .expect("Expected the tilemap to be loaded")
                .textures
                .len();

            spawn_tiles(
                commands,
                board_settings.width,
                board_settings.height,
                tile_count,
                tile_map,
            );
        }
    });
}

fn spawn_tiles(
    mut commands: Commands,
    width: i32,
    height: i32,
    tile_count: usize,
    tile_map: Res<TileTextures>,
) {
    let min_x = (-80 * width) / 2;
    let min_y = (-80 * height) / 2;

    let mut rng = thread_rng();

    for x in 0..width {
        for y in 0..height {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tile_map.clone(),
                transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0)).with_translation(
                    Vec3::new(
                        (min_x + (80 * x) + 40) as f32,
                        (min_y + (80 * y) + 40) as f32,
                        0.0,
                    ),
                ),
                sprite: TextureAtlasSprite {
                    index: rng.gen_range(0..tile_count),
                    ..default()
                },
                ..default()
            });
        }
    }
}
