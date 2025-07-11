#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    let mut row = 1;
    let mut col = 1;
    let mut rd = 1;
    let mut cd = 1;

    loop {
        timer.delay_ms(200u32);
        row += rd;
        match row {
            1 => {
                board.display_pins.row2.set_low().unwrap();
                board.display_pins.row1.set_high().unwrap();
                rd = 1;
                col += cd;
                if col == 5 {
                    cd = -1;
                } else if col == 1 {
                    cd = 1
                }
            }
            2 => {
                board.display_pins.row1.set_low().unwrap();
                board.display_pins.row3.set_low().unwrap();
                board.display_pins.row2.set_high().unwrap();
            }
            3 => {
                board.display_pins.row2.set_low().unwrap();
                board.display_pins.row4.set_low().unwrap();
                board.display_pins.row3.set_high().unwrap();
            }
            4 => {
                board.display_pins.row3.set_low().unwrap();
                board.display_pins.row5.set_low().unwrap();
                board.display_pins.row4.set_high().unwrap();
            }
            5 => {
                board.display_pins.row4.set_low().unwrap();
                board.display_pins.row5.set_high().unwrap();
                rd = -1;
                col += cd;
                if col == 5 {
                    cd = -1;
                } else if col == 1 {
                    cd = 1
                }
            }
            _ => {
                //out of bounds somehow, reset to 1,1
                rd = 1;
                cd = 1;
                row = 1;
                col = 1;
            }
        }
        match col {
            1 => {
                board.display_pins.col1.set_low().unwrap();
                board.display_pins.col2.set_high().unwrap();
                board.display_pins.col3.set_high().unwrap();
                board.display_pins.col4.set_high().unwrap();
                board.display_pins.col5.set_high().unwrap();
            }
            2 => {
                board.display_pins.col1.set_high().unwrap();
                board.display_pins.col2.set_low().unwrap();
                board.display_pins.col3.set_high().unwrap();
                board.display_pins.col4.set_high().unwrap();
                board.display_pins.col5.set_high().unwrap();
            }
            3 => {
                board.display_pins.col1.set_high().unwrap();
                board.display_pins.col2.set_high().unwrap();
                board.display_pins.col3.set_low().unwrap();
                board.display_pins.col4.set_high().unwrap();
                board.display_pins.col5.set_high().unwrap();
            }
            4 => {
                board.display_pins.col1.set_high().unwrap();
                board.display_pins.col2.set_high().unwrap();
                board.display_pins.col3.set_high().unwrap();
                board.display_pins.col4.set_low().unwrap();
                board.display_pins.col5.set_high().unwrap();
            }
            5 => {
                board.display_pins.col1.set_high().unwrap();
                board.display_pins.col2.set_high().unwrap();
                board.display_pins.col3.set_high().unwrap();
                board.display_pins.col4.set_high().unwrap();
                board.display_pins.col5.set_low().unwrap();
            }
            _ => {
                // out of bounds somehow, reset to 1,1
                row = 1;
                col = 1;
                rd = 1;
                cd = 1;
            }
        }
    }
}
