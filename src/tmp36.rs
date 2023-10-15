
use arduino_hal::pac::ADC;
use arduino_hal::hal::Atmega;
use arduino_hal::adc::AdcChannel;
use arduino_hal::adc::Adc;
use arduino_hal::Peripherals;
use arduino_hal::port::mode::Analog;
use arduino_hal::port::mode::Floating;
use arduino_hal::port::mode::Input;
use arduino_hal::port::{Pin, PinOps};

type AnalogPin<A> = Pin<Analog, A>;

pub struct Tmp36<A> {
    sensor: AnalogPin<A>,
    adc: Adc
}

impl<A> Tmp36<A>
where 
	Pin<Input<Floating>, A>: AdcChannel<Atmega, ADC>,
	A: PinOps
{
    pub fn new(mut pin: Pin<Input<Floating>, A>, dp: Peripherals) -> Self {

        let mut adc = Adc::new(dp.ADC, Default::default());
        let temp_sensor = pin.into_analog_input(&mut adc);

        Tmp36 {
            sensor: temp_sensor,
            adc,
        }

    }

    pub fn get_temperature(&mut self) -> f32 {

    	(self.get_voltage() - 0.5) * 100.0

    }

    pub fn get_voltage(&mut self) -> f32 {

    	let reading = self.sensor.analog_read(&mut self.adc);
		(reading as f32)/1024.0 * 5.0

    }

}
