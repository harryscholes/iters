pub mod every;
pub mod repeat;
pub mod times;

#[cfg(test)]
mod tests {
    use super::*;
    use every::EveryIterator;
    use repeat::RepeatIterator;
    use times::TimesIterator;

    #[test]
    fn integration() {
        assert_eq!(
            (1..=5)
                .every(2) // Every second element `(1, 3, 5)`
                .repeat(2) // Repeat each element twice `(1, 1, 3, 3, 5, 5)`
                .times(2) // Repeat the entire iterator twice `(1, 1, 3, 3, 5, 5, 1, 1, 3, 3, 5, 5)`
                .collect::<Vec<i32>>(),
            vec![1, 1, 3, 3, 5, 5, 1, 1, 3, 3, 5, 5]
        )
    }
}
