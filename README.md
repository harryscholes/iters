# iters

Rust iterators and iterator adaptors

```rs
assert_eq!(
    (1..=5)
        .every(2) // Every second element `(1, 3, 5)`
        .times(2) // Repeat the entire iterator twice `(1, 3, 5, 1, 3, 5)`
        .repeat(2) // Repeat each element twice `(1, 1, 3, 3, 5, 5, 1, 1, 3, 3, 5, 5)`
        .collect::<Vec<i32>>(),
    vec![1, 1, 3, 3, 5, 5, 1, 1, 3, 3, 5, 5]
)
```
