use graphics::grid::Grid;


pub struct Tile {
    pub pos: (u32, u32),
    pub bombs_around: u32,
    pub clicked: bool
}


impl Tile {
    pub fn count_bombs(&mut self, bombs: &Vec<(u32, u32)>) {
        let (x, y) = self.pos;
        for i in -1..=1 {
            for j in -1..=1 {
                let around_x = x as i32 + i;
                let around_y = y as i32 + j;
                if around_x >= 0 && around_y >= 0 {
                    if bombs.contains(&(around_x as u32, around_y as u32)) {
                        self.bombs_around += 1;
                    }
                }
            }
        }
    }
}