use std::iter::Peekable;

pub struct Repeat<I: Iterator> {
    iter: Peekable<I>,
    n: usize,
    count: usize,
}

impl<I: Iterator> Repeat<I> {
    pub fn new(iter: I, n: usize) -> Repeat<I> {
        Repeat {
            iter: iter.peekable(),
            n,
            count: 0,
        }
    }
}

impl<I> Iterator for Repeat<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.count += 1;

        if self.count == self.n {
            self.count = 0;
            self.iter.next()
        } else {
            self.iter.peek().cloned()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (mut lower, upper) = self.iter.size_hint();

        if lower > 0 {
            lower *= self.n;
            lower -= self.count;
        }

        match upper {
            Some(_) => (lower, Some(lower)),
            None => (lower, None),
        }
    }
}

impl<I> ExactSizeIterator for Repeat<I>
where
    I: ExactSizeIterator + Iterator,
    I::Item: Clone,
{
}
impl<I> std::iter::FusedIterator for Repeat<I>
where
    I: std::iter::FusedIterator + Iterator,
    I::Item: Clone,
{
}

pub trait RepeatIterator: Iterator {
    fn repeat(self, n: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, n)
    }
}

impl<I: Iterator> RepeatIterator for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_iterator_adapter() {
        assert_eq!(
            (1..3).repeat(2).collect::<Vec<i32>>(),
            Repeat::new(1..3, 2).collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2, 3].into_iter().repeat(2).collect::<Vec<i32>>(),
            Repeat::new(vec![1, 2, 3].into_iter(), 2).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_repeat_iteration() {
        let mut iter = (1..3).repeat(2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_repeat_size_hint() {
        let mut iter = (1..3).repeat(2);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_repeat_len() {
        let mut iter = (1..3).repeat(2);
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_repeat_pipelining() {
        assert_eq!(
            (1..3).repeat(2).map(|x| x * 2).collect::<Vec<i32>>(),
            vec![2, 2, 4, 4]
        );
        assert_eq!(
            (1..3).map(|x| x * 2).repeat(2).collect::<Vec<i32>>(),
            vec![2, 2, 4, 4]
        );
        assert_eq!(
            (1..3)
                .repeat(2)
                .zip((2..4).repeat(2))
                .collect::<Vec<(i32, i32)>>(),
            vec![(1, 2), (1, 2), (2, 3), (2, 3)]
        );
        assert_eq!(
            (1..3).zip(2..4).repeat(2).collect::<Vec<(i32, i32)>>(),
            vec![(1, 2), (1, 2), (2, 3), (2, 3)]
        );
    }
}
