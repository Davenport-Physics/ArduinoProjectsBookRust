#![no_std]
#![no_main]

use arduino_hal::prelude::_void_ResultVoidExt;
use panic_halt as _;


mod triple;
use triple::TriplePins;
use ufmt::uwriteln;
use ufmt_float::uFmt_f32;

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

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let temp_sensor = pins.a0.into_analog_input(&mut adc);
    //let mut pin_cycler = TriplePins::new(pins.d3.into_output(), pins.d4.into_output(), pins.d5.into_output());

    loop {

        let reading = temp_sensor.analog_read(&mut adc);
        let temp_voltage = voltage_to_temp(tmp36_to_voltage(reading));
        uwriteln!(&mut serial, "{}", temp_voltage.convert()).void_unwrap();
        arduino_hal::delay_ms(100);

    }

}

fn tmp36_to_voltage(reading: u16) -> f32 {
    (reading as f32)/1024.0 * 5.0
}

fn voltage_to_temp(voltage: f32) -> f32 {
    (voltage - 0.5) * 100.0
}