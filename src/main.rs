#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board, display::blocking::Display, gpio::SPEAKER, hal::{delay::Delay, gpio::Level, prelude::*, twim, Timer}, pac::twim0::frequency::FREQUENCY_A
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

fn update_board(image: &mut [[u8; 5]; 5]) {
    *image = [
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];
}

fn reset_board(image: &mut [[u8; 5]; 5]) {
    *image = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
}

fn make_sound(speaker: &mut SPEAKER, delay:&mut Delay) {
    speaker.set_high().unwrap();
    delay.delay_us(5000u16);
    speaker.set_low().unwrap();
    delay.delay_us(500u16);
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::Normal,
            lsm303agr::AccelOutputDataRate::Hz10,
        )
        .unwrap();

    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

    let pins = board.display_pins;
    let mut image = [[0; 5]; 5];
    reset_board(&mut image);

    let mut display = Display::new(pins);
    loop {
        rprintln!("showing");
        display.show(&mut timer, image, 1000);
        reset_board(&mut image);
        let status = sensor.accel_status().unwrap();
        if status.xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            rprintln!("{:?}", data.x_mg());
            rprintln!("{:?}", data.y_mg());
            rprintln!("{:?}", data.z_mg());
            if data.x_mg() > 500 {
                rprintln!("updating");
                update_board(&mut image);
                make_sound(&mut speaker, &mut delay);
            }
        }
    }
}
