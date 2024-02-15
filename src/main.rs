#![no_main]
#![no_std]

use cortex_m_rt::entry;
use critical_section_lock_mut::LockMut;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    gpio::SPEAKER,
    hal::{delay::Delay, gpio::Level, prelude::*, timer::Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
    pac::{self, interrupt, TIMER1},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

fn update_board(image: &mut GreyscaleImage) {
    *image = GreyscaleImage::new(&[
        [0, 0, 7, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 7, 0, 0],
    ]);
}

fn reset_board(image: &mut GreyscaleImage) {
    *image = GreyscaleImage::new(&[
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]);
}

fn make_sound(speaker: &mut SPEAKER, delay: &mut Delay) {
    for _ in 0..100 {
        speaker.set_high().unwrap();
        delay.delay_us(500u16);
        speaker.set_low().unwrap();
        delay.delay_us(500u16);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
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
    let mut image = GreyscaleImage::new(&[
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]);

    let display = Display::new(board.TIMER1, board.display_pins);
    DISPLAY.init(display);

    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }

    loop {
        DISPLAY.with_lock(|display| display.show(&image));
        let status = sensor.accel_status().unwrap();
        if status.xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            let x: f32 = data.x_mg() as f32 / 1000f32;
            let y: f32 = data.y_mg() as f32 / 1000f32;
            let z: f32 = data.z_mg() as f32 / 1000f32;
            let magnitude: f32 = (x * x) + (y * y) + (z * z);
            if magnitude < 0.25 {
                rprintln!("{:?}, {:?}, {:?}", data.x_mg(), data.y_mg(), data.z_mg());
                rprintln!("{:?}", magnitude);
                rprintln!("updating");
                make_sound(&mut speaker, &mut delay);
                update_board(&mut image);
            } else {
                reset_board(&mut image);
            }
        }
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
