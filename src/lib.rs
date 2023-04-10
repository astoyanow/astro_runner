#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;
use pluggable_interrupt_os::println;
use x86_64::instructions::random;
use core::{format_args, ops::SubAssign};

#[derive(Copy,Debug,Clone,Eq,PartialEq)]

pub struct Game {
    score_count: isize,
    tick_count: isize,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
}
impl Game {
    pub fn new() -> Self{
        Game { 
            score_count: 0, 
            tick_count: 0,
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT)
        }

    }
    pub fn update_score(&mut self){
        self.score_count += 1;
    }

    pub fn tick(&mut self) {
        self.update_score();
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
pub struct Ship {
    fill: isize
}

impl Default for Ship {
    fn default() -> Self {
        Self { fill: 0 }
    }
}

    fn letter_columns(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.col)
            .take(self.num_letters.a())
            .map(|m| m.a())
    }

    pub fn tick(&mut self) {
        self.clear_current();
        self.update_location();
        self.draw_current();
    }

    fn clear_current(&self) {
        for x in self.letter_columns() {
            plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
        }
    }

    fn update_location(&mut self) {
        self.col += self.dx;
        self.row += self.dy;
    }

    fn draw_current(&self) {
        for (i, x) in self.letter_columns().enumerate() {
            plot(self.letters[i], x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
            //plot('A', x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));

        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        //let future: &mut Ship = self; 
        match key {
            KeyCode::ArrowLeft => {
                self.dx -= 1;
                self.letters = ['<'; BUFFER_WIDTH];
            }
            KeyCode::ArrowRight => {
                self.dx += 1;
                self.letters = ['>'; BUFFER_WIDTH];

            }
            KeyCode::ArrowUp => {
                self.dy -= 1;
                self.letters = ['A'; BUFFER_WIDTH];

            }
            KeyCode::ArrowDown => {
                self.dy += 1;
                self.letters = ['V'; BUFFER_WIDTH];

            }
            _ => {}
        }
        
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            self.letters[self.next_letter.a()] = key;
            self.next_letter += 1;
            self.num_letters = self.num_letters.saturating_add(&ModNumC::new(1));
        }
    }

    pub fn getx(&self) -> ModNumC<usize, BUFFER_WIDTH>{
        return self.dx; 
    }
}


#[derive(PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

pub struct Laser {
    pub beam: [char; 6],
    pub beam_len: ModNumC<usize, 6>,
    pub col: ModNumC<usize, BUFFER_WIDTH>,
    pub row: ModNumC<usize, BUFFER_HEIGHT>,
    pub is_vertical: bool,
    pub direction: Direction,
    pub direction: Direction,
}

impl Laser {
    pub fn new() -> Self {
        Laser {
            beam: ['|'; 6],
            beam_len: ModNumC::new(1),
            col: ModNumC::new(BUFFER_WIDTH/2),
            row: ModNumC::new(BUFFER_HEIGHT/2),
            is_vertical: false,
            direction: Direction::Down,
        }

    }

    fn draw_laser(&mut self) {
        self.orient_laser();
        if self.is_vertical{
            for (i, x) in self.laser_iter_vertical().enumerate() {
                plot(self.beam[i], self.col.a(), x, ColorCode::new(Color::Red, Color::Black));
            }
        } else {
            for (i, x) in self.laser_iter_horizontal().enumerate() {
                plot(self.beam[i], x, self.row.a(), ColorCode::new(Color::Red, Color::Black));
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

    pub fn tick(&mut self){
        self.remove_laser();
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

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(_) => {}
            
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.col -= 1;
            }
            KeyCode::ArrowRight => {
                self.col += 1;
            }
            KeyCode::ArrowUp => {
                self.row -= 1;
            }
            KeyCode::ArrowDown => {
                self.row += 1;
            }
            _ => {}
        }
    }
}
