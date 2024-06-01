struct Decoder<T>
where
    T: Iterator<Item = u8>,
{
    figure_shift: bool,
    source: T,
}

impl<T: Iterator<Item = u8>> Iterator for Decoder<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.source.next() {
            if self.figure_shift {
                match v {
                    _ => [],
                };
            } else {
                match v {
                    _ => {}
                }
            }
        }
        None
    }
}

struct Encoder<T>
where
    T: Iterator<Item = char>,
{
    figure_shift: bool,
    source: T,
}

impl<T: Iterator<Item = char>> Iterator for Encoder<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.source.next() {
            let res = if self.figure_shift {
                match v {
                    '\0' => 0x0,
                    '3' => 0x1,
                    '\n' => 0x2,
                    '–' => 0x3,
                    ' ' => 0x4,
                    '\'' => 0x5,
                    _ => unimplemented!(),
                }
            } else {
                match v {
                    _ => unimplemented!(),
                }
            };

            return Some(res);
        }

        None
    }
}
