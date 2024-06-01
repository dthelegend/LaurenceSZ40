use core::ops::{Deref, DerefMut};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::ita2::EncoderOut;

pub struct Wheel<const N: usize, T> {
    list: [T; N],
    list_pointer: usize,
}

impl<const N: usize, T: Default + Copy + Sized> Wheel<N, T> {
    fn new_default() -> Self {
        Wheel {
            list: [T::default(); N],
            list_pointer: 0
        }
    }
}

impl<const N: usize, T: Copy + Default + Sized> Wheel<N, T>
where
    Standard: Distribution<T>,
{
    fn new_random(rng: &mut impl Rng) -> Self {
        let mut wheel = Self::new_default();

        for i in 0..N {
            wheel.list[i] = rng.gen();
        }

        wheel
    }

    fn step_clockwise(&mut self) {
        self.list_pointer += 1;
        if self.list_pointer >= N {
            self.list_pointer = 0;
        }
    }

    fn read_head(&self) -> T {
        self.list[self.list_pointer]
    }
    fn as_array(&self) -> [T; N] {
        let mut out = [T::default(); N];

        out[..(N - self.list_pointer)].copy_from_slice(&self.list[self.list_pointer..]);
        out[(N - self.list_pointer)..].copy_from_slice(&self.list[..self.list_pointer]);

        out
    }
}

#[repr(transparent)]
pub struct LorenzWheel<const N: usize>(Wheel<N, bool>);

impl <const N: usize> Deref for LorenzWheel<N> {
    type Target = Wheel<N, bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <const N: usize> DerefMut for LorenzWheel<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> LorenzWheel<N> {
    pub fn new_zeroed() -> Self {
        LorenzWheel(Wheel::new_default())
    }

    pub fn new_random(rng: &mut impl Rng) -> Self {
        LorenzWheel(Wheel::new_random(rng))
    }
}

struct LorenzPsiWheels {
    a: LorenzWheel<43>,
    b: LorenzWheel<47>,
    c: LorenzWheel<51>,
    d: LorenzWheel<53>,
    e: LorenzWheel<59>,
}

impl LorenzPsiWheels {
    const N_WHEELS: usize = 5;

    fn new_zeroed() -> Self {
        Self {
            a: LorenzWheel::new_zeroed(),
            b: LorenzWheel::new_zeroed(),
            c: LorenzWheel::new_zeroed(),
            d: LorenzWheel::new_zeroed(),
            e: LorenzWheel::new_zeroed()
        }
    }

    fn new_random(rng: &mut impl Rng) -> Self {
        Self {
            a: LorenzWheel::new_random(rng),
            b: LorenzWheel::new_random(rng),
            c: LorenzWheel::new_random(rng),
            d: LorenzWheel::new_random(rng),
            e: LorenzWheel::new_random(rng)
        }
    }

    fn step_all(&mut self) {
        self.a.step_clockwise();
        self.b.step_clockwise();
        self.c.step_clockwise();
        self.d.step_clockwise();
        self.e.step_clockwise();
    }

    fn read_all(&self) -> u8 {
        (self.a.read_head() as u8) << 4
            + (self.b.read_head() as u8) << 3
            + (self.c.read_head() as u8) << 2
            + (self.d.read_head() as u8) << 1
            + (self.e.read_head() as u8) << 0
    }
}

struct LorenzMuWheels {
    f: LorenzWheel<37>,
    g: LorenzWheel<61>,
}

impl LorenzMuWheels {
    const N_WHEELS: usize = 2;

    fn new_zeroed() -> LorenzMuWheels {
        LorenzMuWheels {
            f: LorenzWheel::new_zeroed(),
            g: LorenzWheel::new_zeroed()
        }
    }
    
    fn new_random(rng : &mut impl Rng) -> LorenzMuWheels {
        LorenzMuWheels {
            f: LorenzWheel::new_random(rng),
            g: LorenzWheel::new_random(rng)
        }
    }
}


struct LorenzChiWheels {
    h: LorenzWheel<41>,
    j: LorenzWheel<31>,
    k: LorenzWheel<29>,
    l: LorenzWheel<26>,
    m: LorenzWheel<23>,
}

impl LorenzChiWheels {
    const N_WHEELS: usize = 5;

    fn new_zeroed() -> Self {
        Self {
            h: LorenzWheel::new_zeroed(),
            j: LorenzWheel::new_zeroed(),
            k: LorenzWheel::new_zeroed(),
            l: LorenzWheel::new_zeroed(),
            m: LorenzWheel::new_zeroed()
        }
    }

    fn new_random(rng: &mut impl Rng) -> Self {
        Self {
            h: LorenzWheel::new_random(rng),
            j: LorenzWheel::new_random(rng),
            k: LorenzWheel::new_random(rng),
            l: LorenzWheel::new_random(rng),
            m: LorenzWheel::new_random(rng)
        }
    }

    fn step_all(&mut self) {
        self.h.step_clockwise();
        self.j.step_clockwise();
        self.k.step_clockwise();
        self.l.step_clockwise();
        self.m.step_clockwise();
    }

    fn read_all(&self) -> u8 {
        (self.h.read_head() as u8) << 4
            + (self.j.read_head() as u8) << 3
            + (self.k.read_head() as u8) << 2
            + (self.l.read_head() as u8) << 1
            + (self.m.read_head() as u8) << 0
    }
}

pub struct LorenzMachine {
    psi: LorenzPsiWheels,
    mu: LorenzMuWheels,
    chi: LorenzChiWheels,
}

impl LorenzMachine {
    const WHEEL_WINDOW : usize = 9;
    pub const OUTPUT_BUFFER_SIZE : usize = LorenzMachine::WHEEL_WINDOW * LorenzPsiWheels::N_WHEELS
        + LorenzMachine::WHEEL_WINDOW * LorenzMuWheels::N_WHEELS
        + LorenzMachine::WHEEL_WINDOW * LorenzChiWheels::N_WHEELS;

    pub fn new_zeroed() -> Self {
        LorenzMachine {
            psi: LorenzPsiWheels::new_zeroed(),
            mu: LorenzMuWheels::new_zeroed(),
            chi: LorenzChiWheels::new_zeroed()
        }
    }
    
    pub fn new_random(rng: &mut impl Rng) -> Self {
        LorenzMachine {
            psi: LorenzPsiWheels::new_random(rng),
            mu: LorenzMuWheels::new_random(rng),
            chi: LorenzChiWheels::new_random(rng)
        }
    }

    pub fn step_machine(&mut self) {
        // Always step psi
        self.psi.step_all();
        // Step chi if both motor wheels
        if self.mu.f.read_head() && self.mu.g.read_head() {
            self.chi.step_all()
        }
        // Step motor g if motor f
        if self.mu.f.read_head() {
            self.mu.g.step_clockwise()
        }
        // Step motor f
        self.mu.f.step_clockwise();
    }

    pub fn encode_at_step(&self, v: u8) -> u8 {
        v ^ self.chi.read_all() ^ self.psi.read_all()
    }

    pub fn draw(&self) -> [bool; LorenzMachine::OUTPUT_BUFFER_SIZE] {

        let mut output_buffer = [false; LorenzMachine::OUTPUT_BUFFER_SIZE];

        output_buffer[..LorenzMachine::WHEEL_WINDOW].copy_from_slice(&self.psi.a.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[..LorenzMachine::WHEEL_WINDOW].reverse();
        
        output_buffer[LorenzMachine::WHEEL_WINDOW..LorenzMachine::WHEEL_WINDOW * 2].copy_from_slice(&self.psi.b.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 2)..LorenzMachine::WHEEL_WINDOW * 3].copy_from_slice(&self.psi.c.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 2)..LorenzMachine::WHEEL_WINDOW * 3].reverse();
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 3)..LorenzMachine::WHEEL_WINDOW * 4].copy_from_slice(&self.psi.d.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 4)..LorenzMachine::WHEEL_WINDOW * 5].copy_from_slice(&self.psi.e.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 4)..LorenzMachine::WHEEL_WINDOW * 5].reverse();
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 5)..LorenzMachine::WHEEL_WINDOW * 6].copy_from_slice(&self.mu.f.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 6)..LorenzMachine::WHEEL_WINDOW * 7].copy_from_slice(&self.mu.g.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 6)..LorenzMachine::WHEEL_WINDOW * 7].reverse();
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 7)..LorenzMachine::WHEEL_WINDOW * 8].copy_from_slice(&self.chi.h.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 8)..LorenzMachine::WHEEL_WINDOW * 9].copy_from_slice(&self.chi.j.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 8)..LorenzMachine::WHEEL_WINDOW * 9].reverse();
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 9)..LorenzMachine::WHEEL_WINDOW * 10].copy_from_slice(&self.chi.k.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 10)..LorenzMachine::WHEEL_WINDOW * 11].copy_from_slice(&self.chi.l.as_array()[..LorenzMachine::WHEEL_WINDOW]);
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 10)..LorenzMachine::WHEEL_WINDOW * 11].reverse();
        
        output_buffer[(LorenzMachine::WHEEL_WINDOW * 11)..].copy_from_slice(&self.psi.b.as_array()[..LorenzMachine::WHEEL_WINDOW]);

        output_buffer
    }
}
