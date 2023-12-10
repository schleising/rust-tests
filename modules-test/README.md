# Using Modules

## Folder Structure

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── library.rs
    ├── library
    │   ├── maths.rs
    │   └── exclaim.rs
    └── main.rs
```

To use a module inside a folder, you need to create a file with the same name as the folder and add `pub mod <file_name>` to it. For example, to use the `maths` module inside the `library` folder, you need to create a file called `library.rs` and add `pub mod maths` to it.

### library.rs

```rust
// library.rs
pub mod maths; // Bring in the maths module
pub mod exclaim; // Bring in the exclaim module
```

### library/maths.rs

```rust
// maths.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### library/exclaim.rs

```rust
// exclaim.rs
pub trait Exclaim {
    fn exclaim(&self) -> String;
}

// Implement Exclaim for any type that implements Display
impl<T: std::fmt::Display> Exclaim for T {
    fn exclaim(&self) -> String {
        format!("{}!", self)
    }
}
```

### main.rs

```rust
// main.rs
mod library; // Needed to use the modules inside the library folder

use library::exclaim::Exclaim; // As this is a trait, we need to use the full path
use library::maths; // We can now use maths::add

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", maths::add(1, 2));
    println!("Hello".exclaim());
}
```

## Notes

- You must have a `.rs` file with the same name as the folder in order to use the modules inside it.
- This file must the contain `pub mod <file_name>` for each module you want to use.
- You have to add `mod <file_name>` to the file in which you want to use the module.


## References

- [Rust Book - 7.5. Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)

