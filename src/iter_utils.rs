pub trait IterUtilsExt: Iterator {
    /// Return the last element from the back that maps to `Some(_)`, or
    /// None if the iterator was exhausted.
    fn ex_rfind_map<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnMut(Self::Item) -> Option<R>,
        Self: DoubleEndedIterator,
    {
        self.rev().find_map(f)
    }
}

impl<I> IterUtilsExt for I where I: Iterator {}
