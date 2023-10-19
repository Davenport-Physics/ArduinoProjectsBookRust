
use arduino_hal::pac::ADC;
use arduino_hal::hal::Atmega;
use arduino_hal::adc::AdcChannel;
use arduino_hal::adc::Adc;
use arduino_hal::port::mode::Analog;
use arduino_hal::port::mode::Floating;
use arduino_hal::port::mode::Input;
use arduino_hal::port::{Pin, PinOps};

pub struct AnalogReader<A> {
    sensor: Pin<Analog, A>,
    adc: Adc
}

impl<A> AnalogReader<A> 
where
	Pin<Analog, A>: AdcChannel<Atmega, ADC>,
    A: PinOps
{
    pub fn new(pin: Pin<Input<Floating>, A>, adc: ADC) -> Self {

        let mut adc = Adc::new(adc, Default::default());
        let sensor = pin.into_analog_input(&mut adc);

        AnalogReader {
            sensor,
            adc,
        }

    }

    pub fn get(&mut self) -> u16 {
        self.sensor.analog_read(&mut self.adc)
    }

}
