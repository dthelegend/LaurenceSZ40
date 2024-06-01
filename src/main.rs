#![no_std]
#![no_main]

mod ita2;
mod lorenz;

use arduino_hal::spi;
use panic_halt as _;

use smart_leds::{SmartLedsWrite, RGB8};

use ws2812_spi::prerendered::Ws2812;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d52.into_output(),
        pins.d51.into_output(),
        pins.d50.into_pull_up_input(),
        pins.d53.into_output(),
        spi::Settings {
            // clock: spi::SerialClockRate::OscfOver8,
            ..Default::default()
        },
    );

    let mut output_buffer = [0; 59 + (3 * 12)];
    let mut data: [RGB8; 5] = [RGB8::default(); 5];
    let empty: [RGB8; 5] = [RGB8::default(); 5];
    let mut ws = Ws2812::new(spi, &mut output_buffer);

    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0x10,
            b: 0,
        };
        data[2] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };
        data[3] = RGB8 {
            r: 0x10,
            g: 0,
            b: 0,
        };

        ws.write(data.iter().cloned()).unwrap();
        arduino_hal::delay_ms(2000);
        ws.write(empty.iter().cloned()).unwrap();
        arduino_hal::delay_ms(2000);
    }
}
