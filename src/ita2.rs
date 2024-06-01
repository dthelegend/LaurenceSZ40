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
                    0x1C => Some('.'),
                    0x1D => Some('/'),
                    0x1E => Some('='),
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

pub struct Encoder<T>
    where
        T: Iterator<Item = char>,
{
    figure_shift: bool,
    source: T,
}

impl <T: Iterator<Item = char>> Encoder<T> {
    pub fn new(source: T) -> Self {
        Self {
            figure_shift: false,
            source
        }
    }
}

const FS : u8 = 0x1B;
const LS : u8 = 0x1F;

pub enum EncoderOut {
    Single(u8),
    ShiftAndChar(u8, u8)
}

impl From<u8> for EncoderOut {
    fn from(value: u8) -> Self {
        EncoderOut::Single(value)
    }
}

impl From<[u8;2]> for EncoderOut {
    fn from([value1, value2]: [u8;2]) -> Self {
        EncoderOut::ShiftAndChar(value1, value2)
    }
}


impl<T: Iterator<Item = char>> Iterator for Encoder<T> {
    type Item = EncoderOut;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.source.next() {
            match v {
                // Doesn't care
                '\0' => Some(0x0.into()),
                '\n' => Some(0x2.into()),
                ' ' => Some(0x4.into()),
                '\r' => Some(0x8.into()),
                // Letter Shift
                'E' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x1].into()
                    } else {
                        0x1.into()
                    }
                }),
                'A' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x3].into()
                    } else {
                        0x3.into()
                    }
                }),
                'S' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x5].into()
                    } else {
                        0x5.into()
                    }
                }),
                'I' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x6].into()
                    } else {
                        0x6.into()
                    }
                }),
                'U' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x7].into()
                    } else {
                        0x7.into()
                    }
                }),
                'D' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x9].into()
                    } else {
                        0x9.into()
                    }
                }),
                'R' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xA].into()
                    } else {
                        0xA.into()
                    }
                }),
                'J' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xB].into()
                    } else {
                        0xB.into()
                    }
                }),
                'N' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xC].into()
                    } else {
                        0xC.into()
                    }
                }),
                'F' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xD].into()
                    } else {
                        0xD.into()
                    }
                }),
                'C' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xE].into()
                    } else {
                        0xE.into()
                    }
                }),
                'K' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0xF].into()
                    } else {
                        0xF.into()
                    }
                }),
                'T' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x10].into()
                    } else {
                        0x10.into()
                    }
                }),
                'Z' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x11].into()
                    } else {
                        0x11.into()
                    }
                }),
                'L' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x12].into()
                    } else {
                        0x12.into()
                    }
                }),
                'W' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x13].into()
                    } else {
                        0x13.into()
                    }
                }),
                'H' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x14].into()
                    } else {
                        0x14.into()
                    }
                }),
                'Y' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x15].into()
                    } else {
                        0x15.into()
                    }
                }),
                'P' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x16].into()
                    } else {
                        0x16.into()
                    }
                }),
                'Q' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x17].into()
                    } else {
                        0x17.into()
                    }
                }),
                'O' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x18].into()
                    } else {
                        0x18.into()
                    }
                }),
                'B' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x19].into()
                    } else {
                        0x19.into()
                    }
                }),
                'G' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x1A].into()
                    } else {
                        0x1A.into()
                    }
                }),
                'M' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x1C].into()
                    } else {
                        0x1C.into()
                    }
                }),
                'X' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x1D].into()
                    } else {
                        0x1D.into()
                    }
                }),
                'V' => Some({
                    if self.figure_shift {
                        self.figure_shift = false;
                        [LS, 0x1E].into()
                    } else {
                        0x1E.into()
                    }
                }),
                // FS
                '3' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x1].into()
                    } else {
                        0x1.into()
                    }
                }),
                '-' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x3].into()
                    } else {
                        0x3.into()
                    }
                }),
                '\'' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x5].into()
                    } else {
                        0x5.into()
                    }
                }),
                '8' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x6].into()
                    } else {
                        0x6.into()
                    }
                }),
                '7' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x7].into()
                    } else {
                        0x7.into()
                    }
                }),
                '\u{0005}' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x9].into()
                    } else {
                        0x9.into()
                    }
                }), // WHO ARE YOU
                '4' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xA].into()
                    } else {
                        0xA.into()
                    }
                }),
                '\u{0007}' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xB].into()
                    } else {
                        0xB.into()
                    }
                }), // BELL
                ',' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xC].into()
                    } else {
                        0xC.into()
                    }
                }),
                '!' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xD].into()
                    } else {
                        0xD.into()
                    }
                }),
                ':' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xE].into()
                    } else {
                        0xE.into()
                    }
                }),
                '(' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0xF].into()
                    } else {
                        0xF.into()
                    }
                }),
                '5' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x10].into()
                    } else {
                        0x10.into()
                    }
                }),
                '+' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x11].into()
                    } else {
                        0x11.into()
                    }
                }),
                ')' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x12].into()
                    } else {
                        0x12.into()
                    }
                }),
                '2' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x13].into()
                    } else {
                        0x13.into()
                    }
                }),
                '£' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x14].into()
                    } else {
                        0x14.into()
                    }
                }),
                '6' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x15].into()
                    } else {
                        0x15.into()
                    }
                }),
                '0' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x16].into()
                    } else {
                        0x16.into()
                    }
                }),
                '1' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x17].into()
                    } else {
                        0x17.into()
                    }
                }),
                '9' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x18].into()
                    } else {
                        0x18.into()
                    }
                }),
                '?' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x19].into()
                    } else {
                        0x19.into()
                    }
                }),
                '&' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x1A].into()
                    } else {
                        0x1A.into()
                    }
                }),
                '.' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x1C].into()
                    } else {
                        0x1C.into()
                    }
                }),
                '/' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x1D].into()
                    } else {
                        0x1D.into()
                    }
                }),
                '=' => Some({
                    if self.figure_shift {
                        self.figure_shift = true;
                        [FS, 0x1E].into()
                    } else {
                        0x1E.into()
                    }
                }),
                _ => self.next()
            }
        } else {
            None
        }
    }
}