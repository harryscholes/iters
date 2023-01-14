#[macro_export]
macro_rules! test_iter_size {
    ($iter:expr, $assertion:expr) => {
        for i in $assertion {
            assert_eq!($iter.size_hint(), (i, Some(i)));
            assert_eq!($iter.len(), i);
            $iter.next();
        }
    };
}
