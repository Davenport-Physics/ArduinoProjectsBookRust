
use arduino_hal::adc::{AdcChannel, AdcOps};
use arduino_hal::pac::ADC;
use arduino_hal::{Adc, Peripherals};
use arduino_hal::port::mode::{Analog, Floating, Input};
use arduino_hal::port::{Pin, PinOps};


pub struct Tmp36<A> {
	sensor: Pin<Analog, A>,
	adc: Adc
}

// Unsure what AdcChannel should be.
// TODO: Finish this out, it's completely broken.
impl<A: PinOps + AdcChannel> Tmp36<A> {

	pub fn new(mut pin: Pin<Analog, A>) -> Tmp36<A> {

    	let dp = Peripherals::take().unwrap();

    	let mut adc = Adc::new(dp.ADC, Default::default());
    	let temp_sensor = pin.into_analog_input(&mut adc);

		Tmp36 {
			sensor: temp_sensor,
			adc
		}

	}

}