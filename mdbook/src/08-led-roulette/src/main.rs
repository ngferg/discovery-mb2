#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut mtx = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut r = 0;
    let mut c = 0;

    let mut dir = Direction::RIGHT;

    loop {
        mtx[r][c] = 1;
        display.show(&mut timer, mtx, 59); // 59 ms sounds good and prime
        display.clear();
        mtx[r][c] = 0;

        match dir {
            Direction::RIGHT => {
                c += 1;
                if c == 4 {
                    dir = Direction::DOWN;
                }
            }
            Direction::DOWN => {
                r += 1;
                if r == 4 {
                    dir = Direction::LEFT;
                }
            }
            Direction::LEFT => {
                c -= 1;
                if c == 0 {
                    dir = Direction::UP;
                }
            }
            Direction::UP => {
                r -= 1;
                if r == 0 {
                    dir = Direction::RIGHT;
                }
            }
        }
    }
}
