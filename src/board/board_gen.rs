use rand::prelude::*;
use std::ops::Index;

#[derive(Clone, PartialEq, Debug)]
pub enum TileType {
    Empty(u8),
    Bomb,
}

#[derive(Clone, Debug)]
pub enum TileState {
    Visable,
    Flagged,
    Hidden,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub state: TileState,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Empty(0),
            state: TileState::Hidden,
        }
    }
}

impl Tile {
    fn is_bomb(&self) -> bool {
        self.tile_type == TileType::Bomb
    }
}

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Default for Board {
    fn default() -> Self {
        Board::new(0, 0)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.tiles[y * self.width + x]
    }
}

const NEIGHBORS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Board {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::default(); width * height],
        }
    }

    fn get(&self, x: i16, y: i16) -> Option<&Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some(&self[(x, y)])
        }
    }

    fn set_bomb(&mut self, x: usize, y: usize) {
        self.tiles[y * self.width + x].tile_type = TileType::Bomb;
    }

    fn calc_neighbors(&mut self, x: usize, y: usize) {
        let neighbor_bombs = NEIGHBORS
            .iter()
            .map(|(neighbor_x, neighbor_y)| {
                (
                    (x as i16) + (*neighbor_x as i16),
                    (y as i16) + (*neighbor_y as i16),
                )
            })
            .filter_map(|(offset_x, offset_y)| self.get(offset_x, offset_y))
            .filter(|tile| tile.is_bomb())
            .count()
            .try_into()
            .expect("The neighbor count should be 8 or lower");

        self.tiles[y * self.width + x].tile_type = TileType::Empty(neighbor_bombs);
    }

    pub fn generate_board(width: usize, height: usize, mine_count: u32) -> Self {
        let mut board: Board = Board::new(width, height);
        let mut placed_bombs = 0;
        let mut rng = thread_rng();

        while placed_bombs < mine_count {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if !board[(x, y)].is_bomb() {
                board.set_bomb(x, y);
                placed_bombs += 1;
            }
        }

        for x in 0..width {
            for y in 0..height {
                if board[(x, y)].is_bomb() {
                    continue;
                }

                board.calc_neighbors(x, y);
            }
        }

        board
    }
}
