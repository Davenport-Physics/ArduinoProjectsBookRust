use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};

pub struct TriplePins<A: PinOps, B: PinOps, C: PinOps> {
    pin_1: Pin<Output, A>,
    pin_2: Pin<Output, B>,
    pin_3: Pin<Output, C>
}

impl<A: PinOps, B: PinOps, C: PinOps> TriplePins<A, B, C> {

    pub fn new(pin_1: Pin<Output, A>, pin_2: Pin<Output, B>, pin_3: Pin<Output, C>) -> TriplePins<A, B, C> {

        TriplePins { 
            pin_1, 
            pin_2, 
            pin_3
        }

    }

    fn set_all_low(&mut self) {

        self.pin_1.set_low();
        self.pin_2.set_low();
        self.pin_3.set_low();

    }

    fn set_all_high(&mut self) {
        self.pin_1.set_high();
        self.pin_2.set_high();
        self.pin_3.set_high();
    }

    fn cycle(&mut self) {
        self.pin_1.set_high();
        arduino_hal::delay_ms(250);

        self.pin_2.set_high();
        arduino_hal::delay_ms(250);

        self.pin_3.set_high();
        arduino_hal::delay_ms(250);
    }

    pub fn show_leds(&mut self, cycle: bool) {

        if cycle {
            self.set_all_low();
            arduino_hal::delay_ms(250);
            self.cycle();
        } else {
            self.set_all_high();
        }

    }

}