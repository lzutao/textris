use crate::block::Block;
use crate::coord::Coord;
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

type Line = Vec<Option<Block>>;
type Cells = Vec<Line>;

pub struct Field {
    cells: Cells,
    width: usize,
    height: usize,
}

fn make_line(width: usize) -> Line {
    iter::repeat(None).take(width).collect()
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height).map(|_| make_line(width)).collect();
        Field {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_in_range(&self, pos: Coord) -> bool {
        let w = self.width as i8;
        let h = self.height as i8;
        0 <= pos.x() && pos.x() < w && 0 <= pos.y() && pos.y() < h
    }

    fn is_above_ceil(&self, pos: Coord) -> bool {
        pos.y() < 0 && 0 <= pos.x() && pos.x() < self.width as i8
    }

    pub fn is_movable(&self, coords: &[Coord]) -> bool {
        coords
            .iter()
            .all(|&c| self.is_above_ceil(c) || self.is_in_range(c) && self[c].is_none())
    }

    pub fn is_reached(&self) -> bool {
        self.cells[0].iter().any(|c| c.is_some())
    }

    pub fn clear_blocks(&mut self, coords: &[Coord]) {
        for &pos in coords {
            if self.is_in_range(pos) {
                self[pos] = None;
            }
        }
    }

    pub fn render_blocks(&mut self, block: Block, coords: &[Coord]) {
        for &pos in coords {
            if self.is_in_range(pos) {
                self[pos] = Some(block);
            }
        }
    }

    pub fn lines_iter(&self) -> Iter<Line> {
        self.cells.iter()
    }

    pub fn get_line(&self, y: usize) -> &Line {
        &self.cells[y]
    }

    pub fn set_line(&mut self, y: usize, line: Line) {
        if y < self.height {
            self.cells[y] = line;
        }
    }

    pub fn delete_line(&mut self, idx: usize) {
        self.cells.remove(idx);
        self.cells.insert(0, make_line(self.width));
    }
}

impl Index<Coord> for Field {
    type Output = Option<Block>;

    fn index(&self, index: Coord) -> &Self::Output {
        if !self.is_in_range(index) {
            return &None;
        }
        let Coord(x, y) = index;
        &self.cells[y as usize][x as usize]
    }
}

impl IndexMut<Coord> for Field {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let Coord(x, y) = index;
        &mut self.cells[y as usize][x as usize]
    }
}
