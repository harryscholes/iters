#[derive(Clone)]
pub struct Times<I: Iterator + Clone> {
    iter: I,
    copy: I,
    n: usize,
}

impl<I> Times<I>
where
    I: Iterator + Clone,
{
    pub fn new(iter: I, n: usize) -> Times<I> {
        Times {
            copy: iter.clone(),
            iter,
            n: n - 1,
        }
    }
}

impl<I> Iterator for Times<I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            Some(x) => return Some(x),
            None => {
                if self.n == 0 {
                    return None;
                }
                self.n -= 1;
                self.iter = self.copy.clone();
                self.iter.next()
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (copy_lower, _) = self.copy.size_hint();
        let (iter_lower, _) = self.iter.size_hint();
        let l = copy_lower * self.n + iter_lower;
        (l, Some(l))
    }
}

impl<I> ExactSizeIterator for Times<I> where I: ExactSizeIterator + Iterator + Clone {}
impl<I> std::iter::FusedIterator for Times<I> where I: std::iter::FusedIterator + Iterator + Clone {}

pub trait TimesIterator: Iterator {
    fn times(self, n: usize) -> Times<Self>
    where
        Self: Sized + Clone,
    {
        Times::new(self, n)
    }
}

impl<I: Iterator> TimesIterator for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_iterator_adapter() {
        assert_eq!(
            (1..3).times(2).collect::<Vec<i32>>(),
            Times::new(1..3, 2).collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2, 3].into_iter().times(2).collect::<Vec<i32>>(),
            Times::new(vec![1, 2, 3].into_iter(), 2).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_times_iteration_n2() {
        let mut iter = (1..3).times(2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_times_iteration_n3() {
        let mut iter = (1..3).times(3);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_times_size_hint_n2() {
        let mut iter = (1..3).times(2);
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
    fn test_times_size_hint_n3() {
        let mut iter = (1..3).times(3);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        iter.next();
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
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_times_size_hint_skip() {
        let iter = (1..3).times(2).skip(1);
        assert_eq!(iter.size_hint(), (3, Some(3)));

        let iter = (1..3).times(2).skip(2);
        assert_eq!(iter.size_hint(), (2, Some(2)));

        let iter = (1..3).times(2).skip(3);
        assert_eq!(iter.size_hint(), (1, Some(1)));

        let iter = (1..3).times(2).skip(4);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let iter = (1..3).times(2).skip(5);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_times_len() {
        let mut iter = (1..3).times(2);
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
    fn test_times_isomorphism() {
        let iter = 1..10;
        assert_eq!(
            iter.clone().times(1).collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
        assert_eq!(
            iter.clone()
                .times(1)
                .times(1)
                .times(1)
                .times(1)
                .times(1)
                .collect::<Vec<i32>>(),
            iter.clone().collect::<Vec<i32>>(),
        );
    }

    #[test]
    fn test_times_pipelining() {
        assert_eq!(
            (1..3).times(2).map(|x| x * 2).collect::<Vec<i32>>(),
            vec![2, 4, 2, 4]
        );
        assert_eq!(
            (1..3).map(|x| x * 2).times(2).collect::<Vec<i32>>(),
            vec![2, 4, 2, 4]
        );
        assert_eq!(
            (1..3)
                .times(2)
                .zip((1..3).times(2))
                .collect::<Vec<(i32, i32)>>(),
            vec![(1, 1), (2, 2), (1, 1), (2, 2)]
        );
        assert_eq!(
            (1..3).zip(1..3).times(2).collect::<Vec<(i32, i32)>>(),
            vec![(1, 1), (2, 2), (1, 1), (2, 2)]
        );
    }
}
