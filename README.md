wrapped-vec
===========

*wrapped-vec* is a Rust crate for auto-generating type definitions and boilerplate code for wrapping 
Vectors in a custom type. It exports a macro `wrapped_vec!` that takes an item type and a wrapper type
name, creating a named wrapper over `Vec` of the given item type. For example, 
`wrapped_vec!(Wrapper(Vec<Item>))` creates `struct Wrapper(Vec<Item>)`.

A large number of useful trait impls are auto-generated, including `Iter`, `IntoIter` & `Expand`, plus a small
number of useful `Vec`-style methods like `len()`, `iter()` & `is_empty()`.

`wrapped_vec!` helps you avoid exposing library implementation details or creating brittle APIs that break when 
plain `Vec` doesn't provide the right functionality any more. Type synonyms give collections a custom name
but don't address these issues. The common workaround of simply wrapping `Vec` with a custom type requires
manually implementing common useful collection traits such as `Iter`, which involves a lot of boilerplate. 
Implementing `Deref` targetting `Vec` provides the basic `Vec` methods, but still requires manual implementation
of collection traits.

## Usage

Add `wrapped-vec` to your `Cargo.toml`:

```
wrapped-vec = "0.1"
```

Import the crate with macros:

```
#[macro_use]
use wrapped_vec;
```

Then define your custom collections and use just like a plain `Vec`:

```
pub struct Task { ... };
wrapped_vec!(TaskBatch(Vec<Task>));

let batch = TaskBatch::from_iter(vec![Task(), Task()]);
for task in batch {
    task.doWork()
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
