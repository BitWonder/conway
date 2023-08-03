use std::{io::{self, Write}, thread, time::Duration};
// for keyboard
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Debug)]

#[derive(Clone)]
struct Game {
    cells : Vec<Vec<State>>
}

impl Game {
    fn new(square_area : usize) -> Game {
        Game { cells : vec![vec![State::Dead; square_area]; square_area] }
    }

    /*
        takes the game and steps it in time buy one following conway's rules
    */
    fn step(&mut self) {
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
        *self = next;
    }

    /*
        finds the number of neighbors a cell has that's alive
    */
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
                if (tx == position_x) && (ty == position_y) {
                    continue;
                }
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
        return count;
    }

    /*
        Says on the can
     */
    pub fn print_game(&self) {
        clear_screen();
        for row in &self.cells {
            for cell in row {
                match cell {
                    State::Alive => { print!( "#" ) }
                    _ => { print!( "." ) }
                }
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum State {
    Alive,
    Dead
}

fn clear_screen() {
    // if windows or unix then clear screen
    if cfg!(target_os = "windows") {
        // clears the windows screen
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        // clears the unix screen
        std::process::Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    }
}

fn main() {
    let mut game = Game::new( 40 );
    game.cells[5][1] = State::Alive;
    game.cells[5][2] = State::Alive;
    game.cells[6][1] = State::Alive;
    game.cells[6][2] = State::Alive;

    game.cells[3][13] = State::Alive;
    game.cells[3][14] = State::Alive;
    game.cells[4][12] = State::Alive;
    game.cells[4][16] = State::Alive;
    game.cells[5][11] = State::Alive;
    game.cells[5][17] = State::Alive;
    game.cells[6][11] = State::Alive;
    game.cells[6][15] = State::Alive;
    game.cells[6][17] = State::Alive;
    game.cells[6][18] = State::Alive;
    game.cells[7][11] = State::Alive;
    game.cells[7][17] = State::Alive;
    game.cells[8][12] = State::Alive;
    game.cells[8][16] = State::Alive;
    game.cells[9][13] = State::Alive;
    game.cells[9][14] = State::Alive;

    game.cells[1][25] = State::Alive;
    game.cells[2][23] = State::Alive;
    game.cells[2][25] = State::Alive;
    game.cells[3][21] = State::Alive;
    game.cells[3][22] = State::Alive;
    game.cells[4][21] = State::Alive;
    game.cells[4][22] = State::Alive;
    game.cells[5][21] = State::Alive;
    game.cells[5][22] = State::Alive;
    game.cells[6][23] = State::Alive;
    game.cells[6][25] = State::Alive;
    game.cells[7][25] = State::Alive;

    game.cells[3][35] = State::Alive;
    game.cells[3][36] = State::Alive;
    game.cells[4][35] = State::Alive;
    game.cells[4][36] = State::Alive;
    let device_state = DeviceState::new();
    'game_loop : loop {
        game.print_game();
        game.step();
        thread::sleep(Duration::from_millis(5));
        let keys: Vec<Keycode> = device_state.get_keys();
        // if q is pressed then quit program
        for key in keys.iter() {
            if key == &Keycode::Q {
                break 'game_loop;
            }
        }
    }
}