#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num, plot_str, clear_screen};
use pc_keyboard::{DecodedKey, KeyCode};
use rand::rngs::SmallRng;
use rand::RngCore;


#[derive(Clone)]
pub struct Game {
    score_count: isize,
    tick_count: usize,
    col: usize,
    row: usize,
    endgame: bool,
    pub ship: Ship,
    pub lasers: [Laser; 25],
    pub rng: SmallRng
}

impl Game {
    pub fn new(seed: SmallRng) -> Self{
        Game { 
            score_count: 0, 
            tick_count: 0,
            col: (BUFFER_WIDTH/2) - 5,
            row: 0,
            endgame: false,
            ship: Ship::new(),
            lasers: [Laser::new(); 25],
            rng: seed
        }
    }

    pub fn key(&mut self, key: DecodedKey){
        match self.endgame {
            false => {self.ship.key(key)},
            true => {
                match key {
                    DecodedKey::RawKey(KeyCode::R) | DecodedKey::Unicode('r') => self.reset_game(),
                    _ => {}
                }
            },
        }
    }

    pub fn tick(&mut self) {
        if !self.endgame {
            self.update_score();
            self.draw_score();
            self.ship.tick();
            for i in 0..self.lasers.len() {
                self.lasers[i].tick(&mut self.rng);
                self.check_collision(self.lasers[i]);
            }
            if self.endgame {
                self.display_reset_msg();
            }
        }
    }

    fn check_collision(&mut self, laser: Laser) {
        if self.ship.get_col() == laser.col && self.ship.get_row() == laser.row {
            self.endgame = true;
        }
    }

    fn display_reset_msg(&self) {
        clear_screen();
        self.draw_score();
        plot_str("Game Over! Press 'r' to restart", (BUFFER_WIDTH/2) - 20, BUFFER_HEIGHT/2, ColorCode::new(Color::White, Color::Black));
    }

    fn draw_score(&self) {
        plot_str("Score: ", self.col, self.row, ColorCode::new(Color::LightGreen, Color::Black));
        plot_num(self.score_count, self.col + 8, self.row, ColorCode::new(Color::LightGreen, Color::Black));
    }

    fn reset_game(&mut self){
        clear_screen();
        self.score_count = 0;
        self.tick_count = 0;
        self.ship = Ship::new();
        self.lasers = [Laser::new(); 25];
        self.endgame = false;
    }

    pub fn update_score(&mut self){
        self.score_count += 1;
    }
    
}

#[derive(Clone, Copy)]
pub struct Ship {
    avatar: char,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    dx: ModNumC<usize, BUFFER_WIDTH>,
    dy: ModNumC<usize, BUFFER_HEIGHT>,
    pub key_strokes: usize
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            avatar: 'A',
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT / 2),
            dx: ModNumC::new(0),
            dy: ModNumC::new(0),
            key_strokes: 0
        }
    }

    pub fn tick(&mut self) {
        self.clear_current();
        self.update_location();
        self.draw_current();
    }

    fn clear_current(&self) {
        plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::Black, Color::Black));
    }

    fn update_location(&mut self) {
        self.col += self.dx;
        self.row += self.dy;
        self.dx = ModNumC::new(0);
        self.dy = ModNumC::new(0);
    }

    fn draw_current(&self) {
        plot(self.avatar, self.col.a(), self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
        self.key_strokes += 1;
    }

    fn handle_raw(&mut self, key: KeyCode) {
        //let future: &mut Ship = self; 
        match key {
            KeyCode::ArrowLeft => {
                self.dx -= 1;
                self.avatar = '<';
            }
            KeyCode::ArrowRight => {
                self.dx += 1;
                self.avatar = '>';
            }
            KeyCode::ArrowUp => {
                self.dy -= 1;
                self.avatar = 'A';
            }
            KeyCode::ArrowDown => {
                self.dy += 1;
                self.avatar = 'V';
            }
            _ => {}
        }
        
    }

    fn handle_unicode(&mut self, key: char) {
        match key {
            'a' => {
                self.dx -= 1;
                self.avatar = '<';
            }
            'd' => {
                self.dx += 1;
                self.avatar = '>';
            }
            'w' => {
                self.dy -= 1;
                self.avatar = 'A';
            }
            's' => {
                self.dy += 1;
                self.avatar = 'V';
            }
            _ => {}
        }
    }

    pub fn get_col(&self) -> usize {
        return self.col.a(); 
    }

    pub fn get_row(&self) -> usize {
        return self.row.a();
    }
}
    


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Clone, Copy, Debug)]
pub struct Laser {
    pub beam: char,    // the laser
    pub col: usize,
    pub row: usize,
    pub is_vertical: bool,  // orientation of laser, if it is up-down or side to side
    pub direction: Direction,   // which way laser is traveling
    pub location_set: bool  // boolean to check if the laser's position has been randomized and set
}

impl Laser {
    pub fn new() -> Self {
        Laser {
            beam: '|',
            col: 0,
            row: 1,
            is_vertical: true,
            direction: Direction::Down,
            location_set: false
        }

    }

    fn draw_laser(&mut self) {
        self.orient_laser();
        if self.is_vertical{
            plot(self.beam, self.col, self.row, ColorCode::new(Color::LightRed, Color::Black));
        } else {
            plot(self.beam, self.col, self.row, ColorCode::new(Color::LightRed, Color::Black));
        }
    }

    fn orient_laser(&mut self) {
        if self.is_vertical {
            self.beam = '|';
        } else {
            self.beam = '_';
        }
    }

    fn randomize_laser_pos(&mut self, rng: &mut SmallRng){
        let mut new_row = 1;
        let mut new_col = 0;
        let mut dir = Direction::Left;
        match 1 + rng.next_u32() as usize % 4 {
            1 => {dir = Direction::Down}
            2 => {dir = Direction::Up}
            3 => {dir = Direction::Left}
            4 => {dir = Direction::Right}
            _ => {}
        }
        match dir {
            Direction::Down => {new_col = 1 + rng.next_u32() as usize % (BUFFER_WIDTH - 1);}
            Direction::Up => {
                new_col = 1 + rng.next_u32() as usize % (BUFFER_WIDTH - 1);
                new_row = BUFFER_HEIGHT - 1;
            }
            Direction::Left => {
                new_row = 1 + rng.next_u32() as usize % (BUFFER_HEIGHT - 1);
                new_col = BUFFER_WIDTH - 1;
            }
            Direction::Right => { new_row = 1 + rng.next_u32() as usize % (BUFFER_HEIGHT - 1);}
        }
        self.col = new_col;
        self.row = new_row;
        self.direction = dir;
        self.set_vertical();
    }

    fn remove_laser(&self) {
        if self.is_vertical{
            plot(' ', self.col, self.row, ColorCode::new(Color::Black, Color::Black));
        } else {
            plot(' ', self.col, self.row, ColorCode::new(Color::Black, Color::Black));
        }
    }

    fn reset_laser(&mut self){
        match self.direction {
            Direction::Up => {
                if self.row == 1 {
                    self.location_set = false;
                }
            },
            Direction::Down => {
                if self.row == BUFFER_HEIGHT - 1 {
                    self.location_set = false;
                }
            },
            Direction::Left => {
                if self.col == 0 {
                    self.location_set = false;
                }
            },
            Direction::Right => {
                if self.col == BUFFER_WIDTH - 1 {
                    self.location_set = false;
                }
            },
        }
    }

    fn set_vertical(&mut self) {
        match self.direction {
            Direction::Up => self.is_vertical = true,
            Direction::Down => self.is_vertical = true,
            _ => self.is_vertical = false
        }
    }

    pub fn tick(&mut self, rng: &mut SmallRng){     
        self.remove_laser();
        if !self.location_set{
            self.randomize_laser_pos(rng);
            self.location_set = true;
        }
        self.update_position();
        self.draw_laser();
        self.reset_laser();
    }

    fn update_position(&mut self) {
        match self.direction {
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
        }
    }

}
