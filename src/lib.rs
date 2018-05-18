extern crate cellular_traits;

mod submissions;
use submissions::wanja::TestWorld as World;
// use submissions::kranki::TurmbräuWorld as World;
// use submissions::nino::GoWorld as World;
// ...

#[cfg(test)]
mod life {
    use super::World;
    use cellular_traits::{rulesets::{GameOfLife, BinaryCell::*}, CellWorld};
    #[test]
    fn blinker() {
        let mut world = World::<GameOfLife>::new(3, 3);
        world.set_cell(0, 1, Live);
        world.set_cell(1, 1, Live);
        world.set_cell(2, 1, Live);
        // ___    _#_    ___
        // ### -> _#_ -> ###
        // ___    _#_    ___
        world.step();
        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(world.get_cell(x, y), if x == 1 { Live } else { Dead });
            }
        }
        world.step_many(42);
        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(world.get_cell(x, y), if x == 1 { Live } else { Dead });
            }
        }
        world.step_many(23);
        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(world.get_cell(x, y), if y == 1 { Live } else { Dead });
            }
        }
    }
}