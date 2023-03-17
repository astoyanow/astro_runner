#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]

pub struct Game {
    score_count: isize,
    tick_count: isize
}
pub struct Ship {
    letters: [char; BUFFER_WIDTH],
    num_letters: ModNumC<usize, BUFFER_WIDTH>,
    next_letter: ModNumC<usize, BUFFER_WIDTH>,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    dx: ModNumC<usize, BUFFER_WIDTH>,
    dy: ModNumC<usize, BUFFER_HEIGHT>
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            letters: ['A'; BUFFER_WIDTH],
            num_letters: ModNumC::new(1),
            next_letter: ModNumC::new(1),
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT / 2),
            dx: ModNumC::new(0),
            dy: ModNumC::new(0)
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
    pub direction: Direction
}

impl Laser {
    pub fn new() -> Self {
        Laser {
            beam: ['|'; 6],
            beam_len: ModNumC::new(2),
            col: ModNumC::new(BUFFER_WIDTH/2),
            row: ModNumC::new(BUFFER_HEIGHT/2),
            is_vertical: true,
            direction: Direction::Down,
        }

    }

    fn draw_laser(&self) {
        for (i, x) in self.laser_iter().enumerate() {
            plot(self.beam[i], x, self.row.a(), ColorCode::new(Color::Green, Color::Black));
        }
    }

    pub fn laser_iter(&self) -> impl Iterator<Item=usize> {
        ModNumIterator::new(self.col)
            .take(self.beam_len.a())
            .map(|m| m.a())
    }

    fn remove_laser(&self) {
        for x in self.laser_iter() {
            plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
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
}
