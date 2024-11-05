pub trait Lcs<Item> {
    fn subsequence(&self) -> Vec<Item>;

    fn len(&self) -> usize
    where
        Self: Sized,
    {
        self.subsequence().len()
    }

    fn is_empty(&self) -> bool
    where
        Self: Sized,
    {
        self.len() == 0
    }
}
