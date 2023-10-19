#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::adc::Adc;
use arduino_hal::simple_pwm::{
    self, 
    Timer1Pwm, 
    Timer2Pwm,
    IntoPwmPin, 
    Prescaler
};

use ufmt::uwriteln;
use ufmt_float::uFmt_f32;

mod triple;
mod tmp36;
mod analog_reader;

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
    let mut adc = Adc::new(dp.ADC, Default::default());

    let red_reader   = pins.a5.into_analog_input(&mut adc);
    let green_reader = pins.a4.into_analog_input(&mut adc);
    let blue_reader  = pins.a3.into_analog_input(&mut adc);

    let mut timer = Timer1Pwm::new(dp.TC1, Prescaler::Prescale256);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale256);

    let mut green_pin = pins.d9.into_output().into_pwm(&mut timer);
    let mut blue_pin = pins.d10.into_output().into_pwm(&mut timer);
    let mut red_pin = pins.d11.into_output().into_pwm(&mut timer2);

    green_pin.enable();
    red_pin.enable();
    blue_pin.enable();

    loop {

        let red_val = (red_reader.analog_read(&mut adc)/4) as u8;
        let green_val = (green_reader.analog_read(&mut adc)/4) as u8;
        let blue_val = (blue_reader.analog_read(&mut adc)/4) as u8;

        green_pin.set_duty(green_val);
        red_pin.set_duty(red_val);
        blue_pin.set_duty(blue_val);

        uwriteln!(&mut serial, "red = {}", red_val).unwrap();
        arduino_hal::delay_ms(5);
    }

}