#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};

pub struct Ship {
    fill: isize
}

impl Default for Ship {
    fn default() -> Self {
        Self { fill: 0 }
    }
}

pub struct Game {
    tick_count: isize,
    ship: Ship,
    lives: isize,
    lasers: [Laser; BUFFER_HEIGHT * BUFFER_WIDTH]

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
