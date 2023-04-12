#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::RngCore;

#[derive(Clone, Copy)]
pub struct Game {
    score_count: isize,
    tick_count: usize,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    pub ship: Ship,
    pub lasers: [Laser; 5]
}

impl Game {
    pub fn new() -> Self{
        Game { 
            score_count: 0, 
            tick_count: 0,
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT),
            ship: Ship::new(),
            lasers: [Laser::new(); 5]
        }

    }
    pub fn update_score(&mut self){
        self.score_count += 1;
    }

    pub fn tick(&mut self, state:u64) {
        self.update_score();
        self.ship.tick();
        self.lasers[self.tick_count % self.lasers.len()].tick(state);
        //self.draw_current();
        
    }

    /* fn draw_current(&self) {
        for i in self.score_count.enumerate(){
            plot(i, self.col.a(), self.row.a(), ColorCode::new(Color::Cyan, Color::Black));

        }
        //plot(self.score_count, self.col.a(), self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
        //plot('A', x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
    }*/
    
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

    pub fn getx(&self) -> ModNumC<usize, BUFFER_WIDTH>{
        return self.dx; 
    }
}
    


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Clone, Copy, Debug)]
pub struct Laser {
    pub beam: [char; 6],    // the laser
    pub beam_len: ModNumC<usize, 6>,    // length of laser
    pub col: ModNumC<usize, BUFFER_WIDTH>,  // column
    pub row: ModNumC<usize, BUFFER_HEIGHT>, // row
    pub is_vertical: bool,  // orientation of laser, if it is up-down or side to side
    pub direction: Direction,   // which way laser is traveling
    pub location_set: bool  // boolean to check if the laser's position has been randomized and set
}

impl Laser {
    pub fn new() -> Self {
        Laser {
            beam: ['|'; 6],
            beam_len: ModNumC::new(1),
            col: ModNumC::new(0),
            row: ModNumC::new(0),
            is_vertical: true,
            direction: Direction::Down,
            location_set: false
        }

    }

    fn draw_laser(&mut self) {
        self.orient_laser();
        if self.is_vertical{
            for (i, x) in self.laser_iter_vertical().enumerate() {
                plot(self.beam[i], self.col.a(), x, ColorCode::new(Color::LightRed, Color::Black));
            }
        } else {
            for (i, x) in self.laser_iter_horizontal().enumerate() {
                plot(self.beam[i], x, self.row.a(), ColorCode::new(Color::LightRed, Color::Black));
            }
        }
    }

    fn laser_iter_horizontal(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.col)
            .take(self.beam_len.a())
            .map(|m| m.a())
    }

    fn laser_iter_vertical(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.row)
            .take(self.beam_len.a())
            .map(|m| m.a())
    }

    fn orient_laser(&mut self) {
        if self.is_vertical {
            self.beam = ['|'; 6];
        } else {
            self.beam = ['_'; 6];
        }
    }

    fn randomize_laser_pos(&mut self, state: u64){
        let mut rng = SmallRng::seed_from_u64(state);
        let mut new_row = 0;
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
                new_row = BUFFER_WIDTH - 1;
            }
            Direction::Left => {
                new_row = 1 + rng.next_u32() as usize % (BUFFER_HEIGHT - 1);
                new_col = BUFFER_HEIGHT - 1;
            }
            Direction::Right => { new_row = 1 + rng.next_u32() as usize % (BUFFER_HEIGHT - 1);}
        }
        self.col = ModNumC::new(new_col);
        self.row = ModNumC::new(new_row);
        self.direction = dir;
        self.set_vertical();
    }

    fn remove_laser(&self) {
        if self.is_vertical{
            for x in self.laser_iter_vertical() {
                plot(' ', self.col.a(), x, ColorCode::new(Color::Black, Color::Black));
            }
        } else {
            for x in self.laser_iter_horizontal() {
                plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
            }
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
                if self.row == BUFFER_HEIGHT - 2 {
                    self.location_set = false;
                }
            },
            Direction::Left => {
                if self.col == 1 {
                    self.location_set = false;
                }
            },
            Direction::Right => {
                if self.col == BUFFER_WIDTH - 2 {
                    self.location_set = false;
                }
            },
        }
    }

    fn set_vertical(&mut self) {
        if self.direction == Direction::Up || self.direction == Direction::Down{
            self.is_vertical = true;
        } else {
            self.is_vertical = false;
        }
    }

    pub fn tick(&mut self, state: u64){     
        if !self.location_set{
            self.randomize_laser_pos(state);
            self.location_set = true;
        }
        self.remove_laser();
        self.reset_laser();
        self.update_position();
        self.draw_laser();
    }

    fn update_position(&mut self) {
        if self.is_vertical{
            if self.direction == Direction::Down{
                self.row += 1
            } else {
                self.row -= 1
            }
        } else {
            if self.direction == Direction::Right {
                self.col += 1
            } else {
                self.col -= 1
            }
        }
    }

}
