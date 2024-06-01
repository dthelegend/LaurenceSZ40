use rand::Rng;
use rand::distributions::{Distribution, Standard};

struct Wheel<const N: usize, T> {
    front: [T; N],
    front_length: usize,
    rear: [T; N],
    rear_length: usize,
}

impl <const N: usize, T : Default + Copy + Sized> Wheel<N, T> {
    fn new_default() -> Self {
        Wheel {
            front: [T::default(); N],
            front_length: N,
            rear: [T::default(); N],
            rear_length: 0
        }
    }
}

impl <const N: usize, T: Copy + Default + Sized> Wheel<N, T>
where
    Standard: Distribution<T>
{
    fn new_random(rng: &mut impl Rng) -> Self {
        let mut wheel = Self::new_default();

        for i in 0..N {
            wheel.front[i] = rng.gen();
        }
        
        wheel
    }

    fn step_clockwise(mut self) -> Self {
        self = self.enforce_invariant();
        
        self.rear[self.rear_length] = self.front[self.front_length - 1];

        self.front_length -= 1;
        
        self
    }
    
    fn enforce_invariant(mut self) -> Self {
        self.front_length = N;
        self.rear_length = 0;
        self.front.copy_from_slice(&self.rear);
        self.front.reverse();
        
        self
    }
    
    fn read_head(&self) -> T {
        self.front[self.front_length]
    }

    fn as_array(&self) -> [T; N] {
        let mut out = [T::default(); N];
        
        out[..self.front_length].copy_from_slice(&self.front[..self.front_length]);
        out[self.front_length..].copy_from_slice(&self.rear[..self.rear_length]);
        
        out
    }
}

struct LorenzWheel<const N : usize> {
    wheel: Wheel<N, bool>
}

impl <const N : usize> LorenzWheel<N> {
    fn new_zeroed() -> Self {
        LorenzWheel {
            wheel: Wheel::new_default()
        }
    }

    fn new_random(rng: &mut impl Rng) -> Self {
        LorenzWheel {
            wheel: Wheel::new_random(rng)
        }
    }
}

struct LorenzPsiWheels {
    a: LorenzWheel<43>,
    b: LorenzWheel<47>,
    c: LorenzWheel<51>,
    d: LorenzWheel<53>,
    e: LorenzWheel<59>
}

struct LorenzMuWheels {
    f: LorenzWheel<37>,
    g: LorenzWheel<61>
}

struct LorenzChiWheels {
    h: LorenzWheel<41>,
    j: LorenzWheel<31>,
    k: LorenzWheel<29>,
    l: LorenzWheel<26>,
    m: LorenzWheel<23>
}

struct LorenzMachine {
    psi: LorenzPsiWheels,
    mu: LorenzMuWheels,
    chi: LorenzChiWheels
}