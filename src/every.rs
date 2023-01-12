pub struct Every<I> {
    iter: I,
    n: usize,
}

impl<I: Iterator> Every<I> {
    pub fn new(iter: I, n: usize) -> Every<I> {
        Every { iter, n }
    }
}

impl<I> Iterator for Every<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        let val = self.iter.next();
        for _ in 1..self.n {
            self.iter.next();
        }
        val
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.iter.size_hint();
        if lower == 0 {
            (0, Some(0))
        } else {
            let l = ((lower - 1) / self.n) + 1;
            (l, Some(l))
        }
    }
}

impl<I> ExactSizeIterator for Every<I>
where
    I: ExactSizeIterator + Iterator,
    I::Item: Clone,
{
}
impl<I> std::iter::FusedIterator for Every<I>
where
    I: std::iter::FusedIterator + Iterator,
    I::Item: Clone,
{
}

pub trait EveryIterator: Iterator {
    fn every(self, n: usize) -> Every<Self>
    where
        Self: Sized,
    {
        Every::new(self, n)
    }
}

impl<I: Iterator> EveryIterator for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_iterator_adapter() {
        assert_eq!(
            (1..9).every(2).collect::<Vec<i32>>(),
            Every::new(1..9, 2).collect::<Vec<i32>>()
        );
        assert_eq!(
            (1..9).every(3).collect::<Vec<i32>>(),
            Every::new(vec![1, 2, 3, 4, 5, 6, 7, 8].into_iter(), 3).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_every_iteration() {
        let mut iter = (1..9).every(3);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_every_size_hint_n2() {
        let mut iter = (1..10).every(2);
        assert_eq!(iter.size_hint(), (5, Some(5)));
        iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_every_size_hint_n3() {
        let mut iter = (1..10).every(3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_every_size_hint_n4() {
        let mut iter = (1..10).every(4);
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_every_size_hint_n5() {
        let mut iter = (1..10).every(5);
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_every_len_n2() {
        let mut iter = (1..10).every(2);
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_every_len_n3() {
        let mut iter = (1..10).every(3);
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_every_len_n4() {
        let mut iter = (1..10).every(4);
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_every_len_n5() {
        let mut iter = (1..10).every(5);
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_every_isomorphism() {
        let iter = 1..10;
        assert_eq!(
            iter.clone().every(1).collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
        assert_eq!(
            iter.clone()
                .every(1)
                .every(1)
                .every(1)
                .every(1)
                .every(1)
                .collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
    }

    #[test]
    fn test_every_pipelining() {
        assert_eq!(
            (1..10).every(3).map(|x| x * 2).collect::<Vec<i32>>(),
            vec![2, 8, 14]
        );
        assert_eq!(
            (1..10).map(|x| x * 2).every(3).collect::<Vec<i32>>(),
            vec![2, 8, 14]
        );
        assert_eq!(
            (1..10)
                .every(3)
                .zip((1..10).every(3))
                .collect::<Vec<(i32, i32)>>(),
            vec![(1, 1), (4, 4), (7, 7)]
        );
        assert_eq!(
            (1..10).zip(1..10).every(3).collect::<Vec<(i32, i32)>>(),
            vec![(1, 1), (4, 4), (7, 7)]
        );
    }
}
