mod board_gen;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::TileTextures;
use board_gen::{Board, TileType};

#[derive(Resource, Deref, DerefMut, Debug)]
struct GameBoard(Board);

fn init_board(mut commands: Commands) {
    commands.insert_resource(GameBoard(Board::default()));
}

#[derive(Default)]
struct BoardSettings {
    size: UVec2,
    mine_ratio: f32,
}

fn generate_board(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    board: ResMut<GameBoard>,
    mut board_settings: Local<BoardSettings>,
    mut tiles: Query<Entity, With<TextureAtlasSprite>>,
    tile_map: Res<TileTextures>,
) {
    egui::Window::new("Generate board").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Board settings:");

        ui.horizontal(|ui| {
            ui.label("Width:");

            ui.add(
                egui::DragValue::new(&mut board_settings.size.x)
                    .speed(0.1)
                    .clamp_range(1..=16),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Height:");

            ui.add(
                egui::DragValue::new(&mut board_settings.size.y)
                    .speed(0.1)
                    .clamp_range(1..=9),
            );
        });

        ui.horizontal(|ui| {
            ui.label("% mines:");

            ui.add(
                egui::DragValue::new(&mut board_settings.mine_ratio)
                    .speed(0.01)
                    .clamp_range(0..=1),
            );
        });

        if ui.button("Generate").clicked() {
            for entity in &mut tiles {
                commands.entity(entity).despawn_recursive();
            }

            spawn_tiles(commands, board, board_settings, tile_map);
        }
    });
}

fn spawn_tiles(
    mut commands: Commands,
    mut board: ResMut<GameBoard>,
    board_settings: Local<BoardSettings>,
    tile_map: Res<TileTextures>,
) {
    let mine_count = ((board_settings.size.x * board_settings.size.y) as f32 * board_settings.mine_ratio).floor() as u32;

    board.0 = Board::generate_board(
        board_settings.size.x as usize,
        board_settings.size.y as usize,
        mine_count,
    );

    let min_x = (-80 * (board_settings.size.x as i32)) / 2;
    let min_y = (-80 * (board_settings.size.y as i32)) / 2;

    for x in 0..(board_settings.size.x as i32) {
        for y in 0..(board_settings.size.y as i32) {
            commands.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: match board[(x as usize, y as usize)].tile_type {
                        TileType::Empty(x) => x as usize,
                        TileType::Bomb => 11,
                    },
                    ..default()
                },
                texture_atlas: tile_map.clone(),
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

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_board)
            .add_system(generate_board);
    }
}