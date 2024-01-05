use crate::seed::IsSeed;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const ALIVE_CELL: &str = "⬛";
const DEAD_CELL: &str = "⬜";
const ALIVE_CELL_PREVIEW: &str = "🟩";
const DEAD_CELL_PREVIEW: &str = "🟦";

pub type Cell = (usize, usize);

#[derive(Debug, Default)]
pub struct Grid {
    pub preview: HashSet<Cell>,
    pub cells: HashSet<Cell>,
    pub width: usize,
    pub height: usize,
    cells_list: Vec<Cell>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                match (self.cells.contains(&(x, y)), self.preview.contains(&(x, y))) {
                    (true, true) => write!(f, "{}", ALIVE_CELL_PREVIEW)?,
                    (true, false) => write!(f, "{}", ALIVE_CELL)?,
                    (false, true) => write!(f, "{}", DEAD_CELL_PREVIEW)?,
                    (false, false) => write!(f, "{}", DEAD_CELL)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let preview = HashSet::new();
        let cells = HashSet::new();
        let cells_list = Vec::new();
        Grid {
            preview,
            cells_list,
            cells,
            width,
            height,
        }
    }

    pub fn seed<S: IsSeed>(&mut self, seed: S, origin: Cell) {
        for cell in seed.cells(origin) {
            self.add_cell(cell);
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        if self.cells.insert(cell) {
            self.preview.clear();
            self.cells_list.push(cell);
        }
    }

    pub fn preview<S: IsSeed>(&mut self, preview: S, origin: Cell) {
        self.preview.clear();
        for cell in preview.cells(origin) {
            self.preview.insert(cell);
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if width == self.width && height == self.height {
            return;
        }

        let mut next_grid = Self::new(width, height);

        self.cells_list
            .iter()
            .filter(|cell| cell.0 < width && cell.1 < height)
            .for_each(|cell| {
                next_grid.add_cell(*cell);
            });

        *self = next_grid;
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.preview.clear();
        self.cells_list.clear();
    }

    pub fn tick(&mut self) {
        let mut next_grid = Self::new(self.width, self.height);

        for cell in &self.cells_list {
            let count = self.count_neighbors(&cell);
            if count == 2 || count == 3 {
                next_grid.add_cell(*cell);
            }

            self.for_each_neighbor_of(cell, |neighbor| {
                if self.count_neighbors(neighbor) == 3 {
                    next_grid.add_cell(*neighbor);
                }
            });
        }

        *self = next_grid
    }

    fn count_neighbors(&self, cell: &Cell) -> usize {
        let mut count = 0;

        self.for_each_neighbor_of(cell, |neighbor| {
            if self.cells.get(neighbor).is_some() {
                count += 1;
            }
        });

        count
    }

    fn for_each_neighbor_of<F>(&self, cell: &Cell, mut callback: F)
    where
        F: FnMut(&Cell),
    {
        let (x_min, x_max) = (cell.0.saturating_sub(1), cell.0.saturating_add(2));
        let (y_min, y_max) = (cell.1.saturating_sub(1), cell.1.saturating_add(2));

        for x_offset in x_min..x_max {
            for y_offset in y_min..y_max {
                if x_offset == cell.0 && y_offset == cell.1 {
                    continue;
                }

                let neighbor = (x_offset, y_offset);
                callback(&neighbor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn test_underpopulation() {
        let mut grid = Grid::new(3, 3);
        grid.add_cell((1, 1)); // Cell has no neighbors

        assert!(grid.cells.contains(&(1, 1)));
        grid.tick();
        assert!(!grid.cells.contains(&(1, 1))); // Cell should die
    }

    #[test]
    fn test_survival() {
        let mut grid = Grid::new(3, 3);
        grid.add_cell((1, 1)); // Cell (1,1) has two neighbors
        grid.add_cell((0, 1));
        grid.add_cell((2, 1));
        grid.add_cell((11, 11)); // Cell (11,11) has three neighbors
        grid.add_cell((10, 10));
        grid.add_cell((10, 11));
        grid.add_cell((12, 11));

        assert!(grid.cells.contains(&(1, 1)));
        assert!(grid.cells.contains(&(11, 11)));
        grid.tick();
        assert!(grid.cells.contains(&(1, 1))); // Cell should survive
        assert!(grid.cells.contains(&(11, 11))); // Cell should survive
    }

    #[test]
    fn test_overpopulation() {
        let mut grid = Grid::new(3, 3);
        grid.add_cell((1, 1)); // Cell (1,1) has > 3 neighbors
        grid.add_cell((0, 0));
        grid.add_cell((0, 1));
        grid.add_cell((1, 0));
        grid.add_cell((2, 2));

        assert!(grid.cells.contains(&(1, 1)));
        grid.tick();
        assert!(!grid.cells.contains(&(1, 1))); // Cell should die
    }

    #[test]
    fn test_reproduction() {
        let mut grid = Grid::new(3, 3);
        grid.add_cell((0, 0));
        grid.add_cell((1, 0));
        grid.add_cell((2, 1)); // Dead cell at (1,1) has three neighbors
        grid.tick();
        assert!(grid.cells.contains(&(1, 1))); // Cell should become alive
    }

    #[test]
    fn test_resize() {
        let mut grid = Grid::new(5, 5);
        grid.add_cell((2, 2));
        grid.add_cell((4, 4));

        assert!(grid.cells.contains(&(2, 2)));
        assert!(grid.cells.contains(&(4, 4)));
        grid.resize(3, 3);
        assert!(grid.cells.contains(&(2, 2)));
        assert!(!grid.cells.contains(&(4, 4))); // Cell should be out of bounds
    }
}
