pub struct Decoder<T>
where
    T: Iterator<Item = u8>,
{
    figure_shift: bool,
    source: T
}

impl <T : Iterator<Item = u8>> Iterator for Decoder<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.source.next() {
            if self.figure_shift {
                match v {
                    0x0 => Some('\0'),
                    0x1 => Some('E'),
                    0x2 => Some('\n'),
                    0x3 => Some('A'),
                    0x4 => Some(' '),
                    0x5 => Some('S'),
                    0x6 => Some('I'),
                    0x7 => Some('U'),
                    0x8 => Some('\r'),
                    0x9 => Some('D'),
                    0xA => Some('R'),
                    0xB => Some('J'),
                    0xC => Some('N'),
                    0xD => Some('F'),
                    0xE => Some('C'),
                    0xF => Some('K'),
                    0x10 => Some('T'),
                    0x11 => Some('Z'),
                    0x12 => Some('L'),
                    0x13 => Some('W'),
                    0x14 => Some('H'),
                    0x15 => Some('Y'),
                    0x16 => Some('P'),
                    0x17 => Some('Q'),
                    0x18 => Some('O'),
                    0x19 => Some('B'),
                    0x1A => Some('G'),
                    0x1B => {
                        self.figure_shift = true;

                        self.next()
                    }
                    0x1C => Some('M'),
                    0x1D => Some('X'),
                    0x1E => Some('V'),
                    0x1F => {
                        self.figure_shift = false;

                        self.next()
                    }
                    _ => self.next()
                }
            } else {
                match v {
                    0x0 => Some('\0'),
                    0x1 => Some('3'),
                    0x2 => Some('\n'),
                    0x3 => Some('-'),
                    0x4 => Some(' '),
                    0x5 => Some('\''),
                    0x6 => Some('8'),
                    0x7 => Some('7'),
                    0x8 => Some('\r'),
                    0x9 => Some('\u{0005}'), // WHO ARE YOU
                    0xA => Some('4'),
                    0xB => Some('\u{0007}'), // BELL
                    0xC => Some(','),
                    0xD => Some('!'),
                    0xE => Some(':'),
                    0xF => Some('('),
                    0x10 => Some('5'),
                    0x11 => Some('+'),
                    0x12 => Some(')'),
                    0x13 => Some('2'),
                    0x14 => Some('£'),
                    0x15 => Some('6'),
                    0x16 => Some('0'),
                    0x17 => Some('1'),
                    0x18 => Some('9'),
                    0x19 => Some('?'),
                    0x1A => Some('&'),
                    0x1B => {
                        self.figure_shift = true;

                        self.next()
                    }
                    0x1C => Some('M'),
                    0x1D => Some('X'),
                    0x1E => Some('V'),
                    0x1F => {
                        self.figure_shift = false;

                        self.next()
                    }
                    _ => self.next()
                }
            }
        } else {
            None
        }
    }
}