pub struct Pairs<I>
where
    I: Iterator + ExactSizeIterator,
{
    iter: I,
    first: Option<I::Item>,
}

impl<I> Pairs<I>
where
    I: Iterator + ExactSizeIterator,
{
    pub fn new(mut iter: I) -> Self {
        Self {
            first: if iter.len() % 2 != 0 {
                iter.next()
            } else {
                None
            },
            iter,
        }
    }
}

impl<I> Iterator for Pairs<I>
where
    I: Iterator + ExactSizeIterator,
{
    type Item = (I::Item, Option<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.first.take() {
            Some((value, None))
        } else if let Some(first) = self.iter.next() {
            // If the iterator has a next element, it will be paired
            let second = self.iter.next();
            Some((first, second))
        } else {
            // End of iteration
            None
        }
    }
}
