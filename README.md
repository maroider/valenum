# valenum
A proc-macro that generates `From<V> for T` and `From<T> for V` impls where `V` is one of Rust's numerical types and `T` is a custom enum with one catch-all variant. It also generates `Serialize` and `Deserialize` impls when the `serde` feature is on.

# Example

```Rust
valenum! {
    pub(crate) enum Region {
        Europe = 0,
        NorthAmerica = 1,
        SouthAmerica = 2,
        Asia = 3,
        Other(i32),
    }
}
```
