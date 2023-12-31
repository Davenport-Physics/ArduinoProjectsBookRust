#![no_std]
#![no_main]

use panic_halt as _;

use ufmt::uwriteln;
use ufmt_float::uFmt_f32;

mod triple;
mod tmp36;

use tmp36::Tmp36;

pub trait EasyUFMT {
    fn convert(self) -> uFmt_f32; 
}

impl EasyUFMT for f32 {

    fn convert(self) -> uFmt_f32 {
        uFmt_f32::Three(self)
    }

}

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut temp_sensor = Tmp36::new(pins.a0, dp.ADC);

    loop {

        uwriteln!(&mut serial, "{}", temp_sensor.get_temperature().convert()).unwrap();
        arduino_hal::delay_ms(100);

    }

}