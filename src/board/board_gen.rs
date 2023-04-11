use rand::prelude::*;
use std::ops::Index;

/// A tile can either be empty with some amount of neighboring bombs, or a bomb itself.
#[derive(Clone, PartialEq, Debug)]
pub enum TileType {
    Empty(u8),
    Bomb,
}

/// While the game is played, the tile can show it's contents, or be flagged, or not show anything.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileState {
    Visable,
    Flagged,
    Hidden,
}

/// A struct that stores a tiles type and it's state.
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
    /// Returns true if the tile is a bomb.
    fn is_bomb(&self) -> bool {
        self.tile_type == TileType::Bomb
    }

    /// Returns a reference to the tile's type.
    pub fn get_type(&self) -> &TileType {
        &self.tile_type
    }
}

/// A struct that contains the tiles that make up a board.
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

/// The offsets from a cell to get its neighbors.
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
    /// Returns the width of the board.
    fn width(&self) -> usize {
        self.width
    }

    /// Returns the width of the board.
    fn height(&self) -> usize {
        self.height
    }

    /// Returns a Board with the given width and height.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::default(); width * height],
        }
    }

    /// Indexes into a board. If the index is out of range [`None`] will be returned.
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

    /// Sets a tile to have a bomb.
    fn set_bomb(&mut self, x: usize, y: usize) {
        self.tiles[y * self.width + x].tile_type = TileType::Bomb;
    }

    pub fn set_state(&mut self, x: usize, y: usize, state: TileState) {
        self.tiles[y * self.width + x].state = state;
    }

    pub fn get_state(&mut self, x: usize, y: usize) -> TileState {
        self.tiles[y * self.width + x].state
    }

    /// Calclulates how many neighboring bombs a tile has.
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

    /// Generates a random board with the given width, height and number of mines.
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
