#![no_std]
#![no_main]

mod ita2;
mod lorenz;

use arduino_hal::prelude::*;
use panic_halt as _;
use rand::SeedableRng;

use smart_leds::{SmartLedsWrite, RGB8};

use ws2812_spi::prerendered::Ws2812;
use crate::ita2::EncoderOut;
use crate::lorenz::LorenzMachine;

const ALL_TEST: [RGB8;LorenzMachine::OUTPUT_BUFFER_SIZE] = [RGB8::new(255, 0, 0); LorenzMachine::OUTPUT_BUFFER_SIZE];
const ALL_OFF : [RGB8;LorenzMachine::OUTPUT_BUFFER_SIZE] = [RGB8::new(0,0,0); LorenzMachine::OUTPUT_BUFFER_SIZE];

#[arduino_hal::entry]
fn main() -> ! {
    const GREETING : &str = "HELLO, WORLD! ";

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

    let (spi, _) = arduino_hal::spi::Spi::new(
        dp.SPI,
        pins.d52.into_output(),
        pins.d51.into_output(),
        pins.d50.into_pull_up_input(),
        pins.d53.into_output(),
        arduino_hal::spi::Settings {
            // clock: spi::SerialClockRate::OscfOver8,
            ..Default::default()
        },
    );

    let mut output_buffer = [0; 40 + (LorenzMachine::OUTPUT_BUFFER_SIZE * 12)];
    let mut ws = Ws2812::new(spi, &mut output_buffer);

    ws.write(ALL_TEST);

    arduino_hal::delay_ms(1000);

    ws.write(ALL_OFF);

    let greeting_loop = GREETING.chars().into_iter().cycle();
    let encoder = ita2::Encoder::new(greeting_loop);
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(57);
    let mut machine = LorenzMachine::new_random(&mut rng);
    
    // let mut machine = LorenzMachine::new_zeroed();

    for v in encoder {
        match v {
            EncoderOut::Single(c1) => {
                // Write out first character
                // let result = machine.encode_at_step(c1);
                
                let out = machine.draw().map(
                    |x| if x { RGB8::new(0, 255, 0) } else { RGB8::new(0,0,255) });
                ws.write(out).unwrap();
            },
            EncoderOut::ShiftAndChar(c1,c2) => {
                // Write out first character
                let result = machine.encode_at_step(c1);
                
                let out = machine.draw().map(
                    |x| if x { RGB8::new(0, 255, 0) } else { RGB8::new(0,0, 255) });
                ws.write(out).unwrap();

                // Wait for effect!
                arduino_hal::delay_ms(1000);

                // Step the encoder
                machine.step_machine();

                // Write out second character
                // let result = machine.encode_at_step(c2);
                
                let out = machine.draw().map(
                    |x| if x { RGB8::new(0, 255, 0) } else { RGB8::new(0,0,255) });
                ws.write(out).unwrap();
            }
        }

        // Wait for effect!
        arduino_hal::delay_ms(1000);

        // Step the encoder
        machine.step_machine();
    }
    
    unreachable!()
}
