
use crate::engine::{grid, prelude::*};
use super::pcg::RandPCG;



/// Tries to avoid everything that isn't an empty space.
/// On tie picks randomly or in a predetermined order based on `RNG`.
pub struct ScaredyCat<const RNG: bool> {
    id: PlayerId,
    pcg: RandPCG
}

impl<const RNG: bool> Bot for ScaredyCat<RNG> {
    fn new(my_player_id: PlayerId) -> Self {
        Self {id: my_player_id, pcg: RandPCG::new_entropic()}
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        fn inner<const RNG: bool>(bot: &mut ScaredyCat<RNG>, game_state: &GameState) -> Option<Direction> {
            // goal is to minimize score
            let mut directs = super::rngesus::not_instant_crash_directions(bot.id, game_state);
            let dir = directs.next()?;
            let mut dirs = vec![dir];
            let mut ans = dir;
            let grid = game_state.current_grid();
            
            fn score(id: PlayerId, grid: &Grid, dir: Direction) -> Option<u8> {
                let pos = grid.player_head_position(id).after_moved(dir)?;
                let mut score = 0u8;
                let mut count = 0u8;
                for neighbor in pos.neighbors() {
                    score = score.saturating_add(
                        match *neighbor.get_cell(grid) {
                            GridCell::Empty => 0,
                            // universally adds 5 to every score.
                            GridCell::Head(..) => 5,
                            GridCell::Tail(..) => 3,
                        }
                    );
                    count = count.wrapping_add(1);
                }
                Some(score.saturating_add(4u8.saturating_sub(count)))
            }

            let mut record = score(bot.id, grid, dir);
            for dir in directs {
                let score = score(bot.id, grid, dir);
                if score < record {
                    if RNG {
                        dirs = vec![dir];
                    } else {
                        ans = dir;
                    }
                    record = score;
                } else if score == record && RNG {
                    dirs.push(dir);
                }
            }
            Some(if RNG {
                dirs[bot.pcg.next_index() % dirs.len()]
            } else {
                ans
            })
        }
        if let Some(dir) = inner(self, game_state) {
            dir
        } else if RNG {
            self.pcg.next_direction()
        } else {
            Direction::NegativeX
        }
    }
}