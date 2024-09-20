pub struct Pairs<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> Pairs<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, Option<&'a T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            None
        } else if self.index == 0 && self.slice.len() % 2 != 0 {
            let first = &self.slice[self.index];
            self.index += 1;
            Some((first, None))
        } else {
            let first = &self.slice[self.index];
            let second = self.slice.get(self.index + 1);
            self.index += 2;
            Some((first, second))
        }
    }
}
