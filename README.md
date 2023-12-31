# arrayinit

A no-dependencies, no-std, stupidly simple and tiny crate for creating const-size arrays dynamically.

By default, when constructing an array `[T; N]`, Rust allows only for two ways to initialize the array.
For example, it allows naming all elements in the array like: `[1, 2, 3, 4]`, or by giving a default value for all of the array entries `["Hello"; 5]`.

There are a few shortcomings to this approach - especially for generically-sized arrays.
In particular, Rust at the moment, is not verbose enough initializing an array with differing values in such a case.

A major problem occurs when `T` is an owned type that doesn't implement `Copy` (for example, `String` type).
Since `N`, the size of the array, is generically defined - we are forced to initialize the array with `[concrete_value; N]`, which wouldn't compile since `concrete_value` is of type `T` which isn't `Copy`.
In order to properly initialize such an array, ugly tricks are required.

This crate comes to solve this problem and allow for a more dynamic initialization very simply.

# Example usage

There are multiple ways to use the `arr!` macro defined in this crate. The most generic way is by supplying a `producer` method, and giving the size of the array.

```rust
let array = arr![|idx| idx * 2; 4];
assert_eq!(array, [0, 2, 4, 6]);
```

Its also possible to omit the size of the array, and let the compiler figure it out on its own

```rust
let array = arr![|idx| idx * 2];

// We need to use the array somewhere to help the compiler understand whats its size should be
assert_eq!(array, [0, 2, 5, 6, 8, 10]);
```

If you feel like it, its also possible to initialize statically all elements:

```rust
let array = arr![1, 2, 3]; // Equivalent to `[1, 2, 3]`
assert_eq!(array, [1, 2, 3]);
```
