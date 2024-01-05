use crate::grid::Cell;

/// A trait for seeding a grid with a pattern of cells.
pub trait IsSeed: std::fmt::Debug {
    fn cells(&self, origin: Cell) -> Vec<Cell>;
}

/// All the possible seeds.
#[derive(Debug)]
pub enum Seed {
    Cell(Cell),
    Still(Still),
    Oscillator(Oscillator),
    Spaceship(Spaceship),
}

impl IsSeed for Seed {
    fn cells(&self, origin: Cell) -> Vec<Cell> {
        match self {
            Seed::Cell(cell) => cell.cells(origin),
            Seed::Still(still) => still.cells(origin),
            Seed::Oscillator(oscillator) => oscillator.cells(origin),
            Seed::Spaceship(spaceship) => spaceship.cells(origin),
        }
    }
}

/// Still lifes are patterns that do not change from one generation to the next.
#[derive(Debug)]
pub enum Still {
    Block,
    Beehive,
    Loaf,
    Boat,
    Tub,
}

/// Oscillators are patterns that return to their original configuration
/// after a finite number of generations.
#[derive(Debug)]
pub enum Oscillator {
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    PentaDecathlon,
}

/// Spaceships are patterns that translate themselves across the grid.
#[derive(Debug)]
pub enum Spaceship {
    Glider,
    LwSpaceship,
    MwSpaceship,
    HwSpaceship,
}

/// Seeds a grid with a single cell.
impl IsSeed for Cell {
    fn cells(&self, origin: Cell) -> Vec<Cell> {
        vec![origin]
    }
}

// ```txt
// o = origin
// * = cell
// ```
impl IsSeed for Still {
    fn cells(&self, origin: Cell) -> Vec<Cell> {
        match self {
            // o *
            // * *
            Still::Block => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0, origin.1.saturating_add(1)),
                (origin.0.saturating_add(1), origin.1.saturating_add(1)),
            ],
            //   o *
            // *     *
            //   * *
            Still::Beehive => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                (origin.0.saturating_add(2), origin.1.saturating_add(1)),
                (origin.0, origin.1.saturating_add(2)),
                (origin.0.saturating_add(1), origin.1.saturating_add(2)),
            ],
            //   o *
            // *     *
            //   *   *
            //     *
            Still::Loaf => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                (origin.0.saturating_add(2), origin.1.saturating_add(1)),
                (origin.0, origin.1.saturating_add(2)),
                (origin.0.saturating_add(2), origin.1.saturating_add(2)),
                (origin.0.saturating_add(1), origin.1.saturating_add(3)),
            ],
            // o *
            // *   *
            //   *
            Still::Boat => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0, origin.1.saturating_add(1)),
                (origin.0.saturating_add(2), origin.1.saturating_add(1)),
                (origin.0.saturating_add(1), origin.1.saturating_add(2)),
            ],
            //   o
            // *   *
            //   *
            Still::Tub => vec![
                origin,
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                (origin.0.saturating_add(1), origin.1.saturating_add(1)),
                (origin.0, origin.1.saturating_add(2)),
            ],
        }
    }
}

// ```txt
// o = origin
// * = cell
// ```
impl IsSeed for Oscillator {
    fn cells(&self, origin: Cell) -> Vec<Cell> {
        match self {
            // o * *
            Oscillator::Blinker => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0.saturating_add(2), origin.1),
            ],
            //   o * *
            // * * *
            Oscillator::Toad => vec![
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0.saturating_add(2), origin.1),
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                (origin.0, origin.1.saturating_add(1)),
                (origin.0.saturating_add(1), origin.1.saturating_add(1)),
            ],
            // o *
            // * *
            //     * *
            //     * *
            Oscillator::Beacon => vec![
                // top left
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0, origin.1.saturating_add(1)),
                (origin.0.saturating_add(1), origin.1.saturating_add(1)),
                // bottom right
                (origin.0.saturating_add(2), origin.1.saturating_add(2)),
                (origin.0.saturating_add(2), origin.1.saturating_add(3)),
                (origin.0.saturating_add(3), origin.1.saturating_add(2)),
                (origin.0.saturating_add(3), origin.1.saturating_add(3)),
            ],
            //    0 1 2 3 4 5 6 7 8 9 0 1 2
            // 0      o * *       * * *
            // 1
            // 2  *         *   *         *
            // 3  *         *   *         *
            // 4  *         *   *         *
            // 5      * * *       * * *
            // 6
            // 7      * * *       * * *
            // 8  *         *   *         *
            // 9  *         *   *         *
            // 0  *         *   *         *
            // 1
            // 2      * * *       * * *
            Oscillator::Pulsar => vec![
                // line 0
                origin,
                (origin.0.saturating_add(1), origin.1),
                (origin.0.saturating_add(2), origin.1),
                (origin.0.saturating_add(6), origin.1),
                (origin.0.saturating_add(7), origin.1),
                (origin.0.saturating_add(8), origin.1),
                // line 1 (empty)
                // line 2
                (origin.0.saturating_sub(2), origin.1.saturating_add(2)),
                (origin.0.saturating_add(3), origin.1.saturating_add(2)),
                (origin.0.saturating_add(5), origin.1.saturating_add(2)),
                (origin.0.saturating_add(10), origin.1.saturating_add(2)),
                // line 3
                (origin.0.saturating_sub(2), origin.1.saturating_add(3)),
                (origin.0.saturating_add(3), origin.1.saturating_add(3)),
                (origin.0.saturating_add(5), origin.1.saturating_add(3)),
                (origin.0.saturating_add(10), origin.1.saturating_add(3)),
                // line 4
                (origin.0.saturating_sub(2), origin.1.saturating_add(4)),
                (origin.0.saturating_add(3), origin.1.saturating_add(4)),
                (origin.0.saturating_add(5), origin.1.saturating_add(4)),
                (origin.0.saturating_add(10), origin.1.saturating_add(4)),
                // line 5
                (origin.0, origin.1.saturating_add(5)),
                (origin.0.saturating_add(1), origin.1.saturating_add(5)),
                (origin.0.saturating_add(2), origin.1.saturating_add(5)),
                (origin.0.saturating_add(6), origin.1.saturating_add(5)),
                (origin.0.saturating_add(7), origin.1.saturating_add(5)),
                (origin.0.saturating_add(8), origin.1.saturating_add(5)),
                // line 6 (empty)
                // line 7
                (origin.0, origin.1.saturating_add(7)),
                (origin.0.saturating_add(1), origin.1.saturating_add(7)),
                (origin.0.saturating_add(2), origin.1.saturating_add(7)),
                (origin.0.saturating_add(6), origin.1.saturating_add(7)),
                (origin.0.saturating_add(7), origin.1.saturating_add(7)),
                (origin.0.saturating_add(8), origin.1.saturating_add(7)),
                // line 8
                (origin.0.saturating_sub(2), origin.1.saturating_add(8)),
                (origin.0.saturating_add(3), origin.1.saturating_add(8)),
                (origin.0.saturating_add(5), origin.1.saturating_add(8)),
                (origin.0.saturating_add(10), origin.1.saturating_add(8)),
                // line 9
                (origin.0.saturating_sub(2), origin.1.saturating_add(9)),
                (origin.0.saturating_add(3), origin.1.saturating_add(9)),
                (origin.0.saturating_add(5), origin.1.saturating_add(9)),
                (origin.0.saturating_add(10), origin.1.saturating_add(9)),
                // line 10
                (origin.0.saturating_sub(2), origin.1.saturating_add(10)),
                (origin.0.saturating_add(3), origin.1.saturating_add(10)),
                (origin.0.saturating_add(5), origin.1.saturating_add(10)),
                (origin.0.saturating_add(10), origin.1.saturating_add(10)),
                // line 11 (empty)
                // line 12
                (origin.0, origin.1.saturating_add(12)),
                (origin.0.saturating_add(1), origin.1.saturating_add(12)),
                (origin.0.saturating_add(2), origin.1.saturating_add(12)),
                (origin.0.saturating_add(6), origin.1.saturating_add(12)),
                (origin.0.saturating_add(7), origin.1.saturating_add(12)),
                (origin.0.saturating_add(8), origin.1.saturating_add(12)),
            ],
            // simplest of its 15 forms
            //   o
            //   *
            // *   *
            //   *
            //   *
            //   *
            //   *
            // *   *
            //   *
            //   *
            Oscillator::PentaDecathlon => vec![
                origin,
                (origin.0, origin.1.saturating_add(1)),
                (origin.0.saturating_sub(1), origin.1.saturating_add(2)),
                (origin.0.saturating_add(1), origin.1.saturating_add(2)),
                (origin.0, origin.1.saturating_add(3)),
                (origin.0, origin.1.saturating_add(4)),
                (origin.0, origin.1.saturating_add(5)),
                (origin.0, origin.1.saturating_add(6)),
                (origin.0.saturating_sub(1), origin.1.saturating_add(7)),
                (origin.0.saturating_add(1), origin.1.saturating_add(7)),
                (origin.0, origin.1.saturating_add(8)),
                (origin.0, origin.1.saturating_add(9)),
            ],
        }
    }
}

// ```txt
// o = origin
// * = cell
// ```
impl IsSeed for Spaceship {
    fn cells(&self, origin: Cell) -> Vec<Cell> {
        match self {
            //   o
            // *
            // * * *
            Spaceship::Glider => vec![
                origin,
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                (origin.0.saturating_sub(1), origin.1.saturating_add(2)),
                (origin.0, origin.1.saturating_add(2)),
                (origin.0.saturating_add(1), origin.1.saturating_add(2)),
            ],
            // 0 1 2 3 4
            //   o     *
            // *
            // *       *
            // * * * *
            Spaceship::LwSpaceship => vec![
                // line 0
                origin,
                (origin.0.saturating_add(3), origin.1),
                // line 1
                (origin.0.saturating_sub(1), origin.1.saturating_add(1)),
                // line 2
                (origin.0.saturating_sub(1), origin.1.saturating_add(2)),
                (origin.0.saturating_add(3), origin.1.saturating_add(2)),
                // line 3
                (origin.0.saturating_sub(1), origin.1.saturating_add(3)),
                (origin.0, origin.1.saturating_add(3)),
                (origin.0.saturating_add(1), origin.1.saturating_add(3)),
                (origin.0.saturating_add(2), origin.1.saturating_add(3)),
            ],
            // 0 1 2 3 4 5
            //     o
            // *       *
            //           *
            // *         *
            //   * * * * *
            Spaceship::MwSpaceship => vec![
                // line 0
                origin,
                // line 1
                (origin.0.saturating_sub(2), origin.1.saturating_add(1)),
                (origin.0.saturating_add(2), origin.1.saturating_add(1)),
                // line 2
                (origin.0.saturating_add(3), origin.1.saturating_add(2)),
                // line 3
                (origin.0.saturating_sub(2), origin.1.saturating_add(3)),
                (origin.0.saturating_add(3), origin.1.saturating_add(3)),
                // line 4
                (origin.0.saturating_sub(1), origin.1.saturating_add(4)),
                (origin.0, origin.1.saturating_add(4)),
                (origin.0.saturating_add(1), origin.1.saturating_add(4)),
                (origin.0.saturating_add(2), origin.1.saturating_add(4)),
                (origin.0.saturating_add(3), origin.1.saturating_add(4)),
            ],
            // 0 1 2 3 4 5 6
            //     o *
            // *         *
            //             *
            // *           *
            //   * * * * * *
            Spaceship::HwSpaceship => vec![
                // line 0
                origin,
                (origin.0.saturating_add(1), origin.1),
                // line 1
                (origin.0.saturating_sub(2), origin.1.saturating_add(1)),
                (origin.0.saturating_add(3), origin.1.saturating_add(1)),
                // line 2
                (origin.0.saturating_add(4), origin.1.saturating_add(2)),
                // line 3
                (origin.0.saturating_sub(2), origin.1.saturating_add(3)),
                (origin.0.saturating_add(4), origin.1.saturating_add(3)),
                // line 4
                (origin.0.saturating_sub(1), origin.1.saturating_add(4)),
                (origin.0, origin.1.saturating_add(4)),
                (origin.0.saturating_add(1), origin.1.saturating_add(4)),
                (origin.0.saturating_add(2), origin.1.saturating_add(4)),
                (origin.0.saturating_add(3), origin.1.saturating_add(4)),
                (origin.0.saturating_add(4), origin.1.saturating_add(4)),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;
    use std::collections::HashSet;

    #[test]
    fn test_still_block_seed() {
        let mut grid = Grid::new(5, 5);
        grid.seed(Still::Block, (0, 0));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (0, 0), (1, 0), 
            (0, 1), (1, 1)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_still_beehive_seed() {
        let mut grid = Grid::new(7, 7);
        grid.seed(Still::Beehive, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2),  (3, 2),
            (1, 3),                  (4, 3), 
                    (2, 4),  (3, 4)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_still_loaf_seed() {
        let mut grid = Grid::new(7, 7);
        grid.seed(Still::Loaf, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2), (3, 2), 
            (1, 3),                 (4, 3), 
                    (2, 4),         (4, 4), 
                            (3, 5)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_still_boat_seed() {
        let mut grid = Grid::new(5, 5);
        grid.seed(Still::Boat, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (2, 2), (3, 2), 
            (2, 3),         (4, 3), 
                    (3, 4), 
            
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_still_tub_seed() {
        let mut grid = Grid::new(5, 5);
        grid.seed(Still::Tub, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2), 
            (1, 3),         (3, 3),
                    (2, 4),
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_oscillator_blinker_seed() {
        let mut grid = Grid::new(5, 5);
        grid.seed(Oscillator::Blinker, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (2, 2), (3, 2), (4, 2)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_oscillator_toad_seed() {
        let mut grid = Grid::new(6, 6);
        grid.seed(Oscillator::Toad, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2), (3, 2), (4, 2), 
            (1, 3), (2, 3), (3, 3)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_oscillator_beacon_seed() {
        let mut grid = Grid::new(6, 6);
        grid.seed(Oscillator::Beacon, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (2, 2), (3, 2), 
            (2, 3), (3, 3), 
                            (4, 4), (5, 4), 
                            (4, 5), (5, 5)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_oscillator_pulsar_seed() {
        let mut grid = Grid::new(17, 17);
        grid.seed(Oscillator::Pulsar, (2, 2));

        //    0 1 2 3 4 5 6 7 8 9 0 1 2
        // 2      o * *       * * *
        // 3
        // 4  *         *   *         *
        // 5  *         *   *         *
        // 6  *         *   *         *
        // 7      * * *       * * *
        // 8
        // 9      * * *       * * *
        // 0  *         *   *         *
        // 1  *         *   *         *
        // 2  *         *   *         *
        // 3
        // 4      * * *       * * *
        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (2, 2), (3, 2), (4, 2), (8, 2), (9, 2), (10, 2),
            // line 3 empty
            (0, 4), (5, 4), (7, 4), (12, 4),
            (0, 5), (5, 5), (7, 5), (12, 5),
            (0, 6), (5, 6), (7, 6), (12, 6),
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            // line 8 empty
            (2, 9), (3, 9), (4, 9), (8, 9), (9, 9), (10, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),
            (0, 11), (5, 11), (7, 11), (12, 11),
            (0, 12), (5, 12), (7, 12), (12, 12),
            // line 13 empty
            (2, 14), (3, 14), (4, 14), (8, 14), (9, 14), (10, 14), 
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_oscillator_penta_decathlon_seed() {
        let mut grid = Grid::new(13, 13);
        grid.seed(Oscillator::PentaDecathlon, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                (2, 2),
                (2, 3),
            (1, 4), (3, 4),
                (2, 5),
                (2, 6),
                (2, 7),
                (2, 8),
            (1, 9), (3, 9),
                (2, 10),
                (2, 11),
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_spaceship_glider_seed() {
        let mut grid = Grid::new(5, 5);
        grid.seed(Spaceship::Glider, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2),
            (1, 3),
            (1, 4), (2, 4), (3, 4)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_spaceship_lw_spaceship_seed() {
        let mut grid = Grid::new(7, 7);
        grid.seed(Spaceship::LwSpaceship, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                    (2, 2),                (5, 2),
            (1, 3),
            (1, 4),                        (5, 4),
            (1, 5), (2, 5), (3, 5), (4, 5),
            
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_spaceship_mw_spaceship_seed() {
        let mut grid = Grid::new(7, 7);
        grid.seed(Spaceship::MwSpaceship, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                            (2, 2),
            (0, 3),                         (4, 3),
                                                    (5, 4),
            (0, 5),                                 (5, 5),
                    (1, 6), (2, 6), (3, 6), (4, 6), (5, 6),
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_spaceship_hw_spaceship_seed() {
        let mut grid = Grid::new(7, 7);
        grid.seed(Spaceship::HwSpaceship, (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
                            (2, 2), (3, 2),
            (0, 3),                                 (5, 3),
                                                            (6, 4),
            (0, 5),                                         (6, 5),
                    (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6),
        ]);

        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn test_single_cell() {
        let mut grid = Grid::new(5, 5);
        grid.seed((0, 0), (2, 2));

        #[rustfmt::skip]
        let expected_cells = HashSet::from([
            (2, 2)
        ]);

        assert_eq!(grid.cells, expected_cells);
    }
}
