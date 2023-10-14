#![no_std]
#![no_main]

use panic_halt as _;


mod triple;
use triple::TriplePins;

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let switch = pins.d2;
    let mut pin_cycler = TriplePins::new(pins.d3.into_output(), pins.d4.into_output(), pins.d5.into_output());
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
