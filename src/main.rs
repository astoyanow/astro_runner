#![no_std]
#![no_main]

use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::{HandlerTable, println};
use pluggable_interrupt_os::vga_buffer::clear_screen;
use astro_runner::{Laser, Ship, Direction};
use crossbeam::atomic::AtomicCell;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(startup)
        .cpu_loop(cpu_loop)
        .start()
}

static LAST_KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);
static TICKS: AtomicCell<usize> = AtomicCell::new(0);

fn cpu_loop() -> ! {
    let mut last_tick = 0;
    let mut kernel = Laser::new();
    let mut player: Ship = Ship::new();
    loop {
        //println!("{last_tick}");
        if last_tick % 3 == 0 {
            kernel.is_vertical = false;
            kernel.beam = ['_'; 6];
            kernel.direction = Direction::Right;
            
        }
        if let Some(key) = LAST_KEY.load() {
            LAST_KEY.store(None);
            player.key(key);
        }
        let current_tick = TICKS.load();
        if current_tick > last_tick {
            last_tick = current_tick;
            kernel.tick();
            player.tick();
        }
    }
}

fn tick() {
    TICKS.fetch_add(1);
}

fn key(key: DecodedKey) {
    LAST_KEY.store(Some(key));
}

fn startup() {
    clear_screen();
}