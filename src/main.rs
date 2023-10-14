#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};

use panic_halt as _;

struct TriplePins<A: PinOps, B: PinOps, C: PinOps> {
    pin_1: Pin<Output, A>,
    pin_2: Pin<Output, B>,
    pin_3: Pin<Output, C>
}

impl<A: PinOps, B: PinOps, C: PinOps> TriplePins<A, B, C> {

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

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let switch = pins.d2;
    let mut pin_cycler = TriplePins {
        pin_1: pins.d3.into_output(),
        pin_2: pins.d4.into_output(),
        pin_3: pins.d5.into_output()
    };

    let mut cycle: bool = false;
    loop {

        if switch.is_high() {
            cycle = !cycle;
            arduino_hal::delay_ms(500);
        }

        pin_cycler.show_leds(cycle);
        arduino_hal::delay_ms(100);

    }
}
