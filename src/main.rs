#![no_std]
#![no_main]

use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use astro_runner::Game;
use crossbeam::atomic::AtomicCell;
use rand::SeedableRng;
use rand::rngs::SmallRng;

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
    let rng = SmallRng::seed_from_u64(4);
    let mut game: Game = Game::new(rng); 
    loop {
        if let Some(key) = LAST_KEY.load() {
            LAST_KEY.store(None);
            game.key(key);
        }
        let current_tick = TICKS.load();
        if current_tick > last_tick {
            last_tick = current_tick;
            game.tick();
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