#[derive(Clone)]
pub struct Repeat<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    el: Option<I::Item>,
    n: usize,
    count: usize,
}

impl<I> Repeat<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iter: I, n: usize) -> Repeat<I> {
        Repeat {
            iter,
            el: None,
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
        if self.count == 0 {
            self.el = self.iter.next();
            if self.el.is_none() {
                return None;
            }
            self.count = self.n;
        }

        self.count -= 1;
        self.el.clone()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.iter.size_hint();
        let l = lower * self.n + self.count;
        (l, Some(l))
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
        Self::Item: Clone,
    {
        Repeat::new(self, n)
    }
}

impl<I: Iterator> RepeatIterator for I {}

#[cfg(test)]
mod tests {
    use crate::test_iter_size;

    use super::*;

    #[test]
    fn test_iterator_adapter() {
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
    fn test_iteration() {
        let mut iter = (1..3).repeat(2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_size_n3() {
        test_iter_size!((1..3).repeat(3), 6..=0);
    }

    #[test]
    fn test_size_n2() {
        test_iter_size!((1..4).repeat(2), 8..=0);
    }

    #[test]
    fn test_isomorphism() {
        let iter = 1..10;
        assert_eq!(
            iter.clone().repeat(1).collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
        assert_eq!(
            iter.clone()
                .repeat(1)
                .repeat(1)
                .repeat(1)
                .collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
    }

    #[test]
    fn test_pipelining() {
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
