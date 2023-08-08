# enum-filter

`enum-filter` provides a macro that will generate "filter" methods for iterators over an enum  
It does so by creating a trait `EnumNameFilter` with the same visibilty as the enum  
For variants with named fields it will create a struct `VariantNameData`, also with the same visibility as the enum  

```rust
#[enum_filter]
enum Example {
    Variant1,
    Variant2(u8),
    Variant3 { data: u8 },
}

fn main() {
    let mut test_vec = vec![
        Example::Variant1,
        Example::Variant2(2),
        Example::Variant3 { data: 3 },
    ];

    assert_eq!(
        test_vec.iter().filter_variant1().collect::<Vec<_>>(),
        vec![()]
    );

    assert_eq!(
        test_vec.iter_mut().filter_variant2().collect::<Vec<_>>(),
        vec![&mut 2]
    );

    assert_eq!(
        test_vec
            .into_iter()
            .filter_variant3()
            .map(|v| v.data)
            .collect::<Vec<_>>(),
        vec![3]
    );
}
```
