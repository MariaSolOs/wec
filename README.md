# WEC

A collection of useful Rust macros for creating `vec!`s.

## `bec!`

Tired of writing `vec![ Box::new(1), ... Box::new(100) ]`? Me too. So here's a macro that does it for you.

```rust
let v = bec![1, 2, 3];
assert_eq!(v, vec![Box::new(1), Box::new(2), Box::new(3)]);
```

## `vinto!`

Calls `.into()` on each element.

```rust
let v: Vec<String> = vinto!["foo", String::from("bar")];
assert_eq!(v, vec![String::from("foo"), String::from("bar")]);
```