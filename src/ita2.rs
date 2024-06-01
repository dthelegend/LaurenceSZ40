struct Decoder<T>
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
                    _ => []
                };
            } else {
                match v { _ => {} }
            }
        }
        None
    }
}