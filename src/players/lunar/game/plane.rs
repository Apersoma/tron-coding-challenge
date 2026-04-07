
use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Plane {
    pub grid: [[bool; GRID_SIZE]; GRID_SIZE], 
    pub x: Pos,
    pub o: Pos
}

impl Plane {
    #[inline]
    pub fn step(&mut self, x_vel: Vel, o_vel: Vel) -> Option<GameEnd> {
        let (x_wish_x, mut x_crash) = self.x.0.overflowing_add_signed(x_vel.0);
        x_crash |= x_wish_x >= GRID_SIZE;
        let (x_wish_y, x_crash_y) = self.x.1.overflowing_add_signed(x_vel.1);
        x_crash |= x_crash_y;
        x_crash |= x_wish_y >= GRID_SIZE;
        let x_wish = Pos(x_wish_x, x_wish_y);

        let (o_wish_x, mut o_crash) = self.o.0.overflowing_add_signed(o_vel.0);
        o_crash |= o_wish_x >= GRID_SIZE;
        let (o_wish_y, o_crash_y) = self.o.1.overflowing_add_signed(o_vel.1);
        o_crash |= o_crash_y;
        o_crash |= o_wish_y >= GRID_SIZE;
        let o_wish = Pos(o_wish_x, o_wish_y);

        let head_to_head = (x_wish_x == o_wish_x) & (x_wish_y == o_wish_y);

        // MUST BE `||`. Short circuit prevents oob.
        let x_crash = x_crash || mem::replace(&mut self.grid[x_wish_x][x_wish_y], true);
        let o_crash = o_crash || mem::replace(&mut self.grid[o_wish_x][o_wish_y], true);
        self.x = x_wish;
        self.o = o_wish;
        
        if (x_crash | o_crash) | head_to_head {
            Some(
                if (x_crash & o_crash) | head_to_head {
                    GameEnd::Tie
                } else if x_crash {
                    GameEnd::WinO
                } else {
                    GameEnd::WinX
                }
            )
        } else {
            None
        }
    }

    #[inline]
    /// True on crash
    pub fn step_x(&mut self, x_vel: Vel) -> bool {
        let (x_wish_x, mut x_crash) = self.x.0.overflowing_add_signed(x_vel.0);
        x_crash |= x_wish_x >= GRID_SIZE;
        let (x_wish_y, x_crash_y) = self.x.1.overflowing_add_signed(x_vel.1);
        x_crash |= x_crash_y;
        x_crash |= x_wish_y >= GRID_SIZE;
        let x_wish = Pos(x_wish_x, x_wish_y);

        // MUST BE `||`. Short circuit prevents oob.
        let x_crash = x_crash || mem::replace(&mut self.grid[x_wish_x][x_wish_y], true);
        self.x = x_wish;
        
        x_crash
    }

    #[inline]
    /// True on crash
    pub fn step_o(&mut self, o_vel: Vel) -> bool {
        let (o_wish_x, mut o_crash) = self.o.0.overflowing_add_signed(o_vel.0);
        o_crash |= o_wish_x >= GRID_SIZE;
        let (o_wish_y, o_crash_y) = self.o.1.overflowing_add_signed(o_vel.1);
        o_crash |= o_crash_y;
        o_crash |= o_wish_y >= GRID_SIZE;
        let o_wish = Pos(o_wish_x, o_wish_y);

        // MUST BE `||`. Short circuit prevents oob.
        let o_crash = o_crash || mem::replace(&mut self.grid[o_wish_x][o_wish_y], true);
        self.o = o_wish;
        
        o_crash
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for row in (0..GRID_SIZE).rev() {
            for col in 0..GRID_SIZE {
                string += if self.o == Pos(row, col) {
                    "O"
                } else if self.x == Pos(row, col) {
                    "X"
                } else if self.grid[row][col] { 
                    "#" 
                } else {
                    "." 
                };
            }
            string += "\n";
        }
        write!(f, "{}", string)
    }
}