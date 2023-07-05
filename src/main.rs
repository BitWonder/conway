#[derive(Debug)]
struct Game {
    cells : Vec<Vec<State>>
}

impl Game {
    fn new(square_area : usize) -> Game {
        Game { cells : vec![vec![State::Dead; square_area]; square_area] }
    }

    fn step(self) -> Game {
        let mut next = Game::new( self.cells.len() );
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                let alive_count = self.count(x as isize, y as isize);
                match self.cells[x][y] {
                    State::Alive => {
                        match alive_count {
                            2 | 3 => {
                                next.cells[x][y] = State::Alive;
                            }
                            _ => {
                                next.cells[x][y] = State::Dead;
                            }
                        }
                    }
                    State::Dead => {
                        match alive_count {
                            3 => {
                                next.cells[x][y] = State::Alive;
                            }
                            _ => {
                                next.cells[x][y] = State::Dead;
                            }
                        }
                    }
                }
            }
        }
        return next;
    }

    fn count(&self, position_x : isize, position_y : isize) -> u64 {
        let mut count : u64 = 0;
        for x in (position_x - 1) as isize ..= (position_x + 1) as isize {
            for y in (position_y - 1) as isize ..= (position_y + 1) as isize {
                let mut tx = x;
                let mut ty = y;
                if x < 0 { tx = (self.cells.len() - 1) as isize }
                else if x == self.cells.len() as isize { tx = 0 }
                if y < 0 { ty = (self.cells.len() - 1) as isize }
                else if y == self.cells.len() as isize { ty = 0 }
                if (tx != position_x) && (ty != position_y) {
                    match self.cells[tx as usize][ty as usize] {
                        State::Alive => {
                            count += 1;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
        return count;
    }
}

#[derive(PartialEq, Clone, Debug)]
enum State {
    Alive,
    Dead
}

fn main() {
    let mut game = Game::new( 5 );
    let mut steps = 0;
    game.cells[0][1] = State::Alive;
    game.cells[1][1] = State::Alive;
    game.cells[2][1] = State::Alive;
    'game_loop : loop {
        if steps > 2 { break 'game_loop; }
        println!("{:?} \n \n \n", game);
        game = game.step();
        steps += 1;
    }
}