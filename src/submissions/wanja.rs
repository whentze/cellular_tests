use cellular_traits::{CellWorld, RuleSet};

pub(crate) struct TestWorld<R: RuleSet> {
    die_einzige_zelle: R::Cell,
}

impl<R: RuleSet> CellWorld<R> for TestWorld<R> {
    fn new(width: usize, height: usize) -> Self {
        if width == 1 && height == 1 {
            TestWorld {
                die_einzige_zelle: R::Cell::default(),
            }
        } else {
            // TODO: Schaise ich muss ja irgendwie mehr als eine einzige Zelle speichern
            unimplemented!()
        }
    }

    fn width(&self) -> usize {
        1
    }
    fn height(&self) -> usize {
        1
    }

    fn set_cell(&mut self, _x: usize, _y: usize, value: R::Cell) {
        self.die_einzige_zelle = value;
    }
    
    fn get_cell(&self, _x: usize, _y: usize) -> R::Cell {
        self.die_einzige_zelle
    }

    fn step(&mut self) {
        let d = R::Cell::default();
        self.die_einzige_zelle = R::step([[d, d, d], [d, self.die_einzige_zelle, d], [d, d, d]]);
    }
}