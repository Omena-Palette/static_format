# Const Format

A simple macro which just makes concat! more ergonomic / easier to read.

```rust
use static_format::const_format;

macro_rules! period {
    () => {'.'}
}

fn main() {
    let formatted = const_format!("{}, there are {} formatted {}{}", "Hello", 4, "arguments", period!());
    assert_eq!(formatted, "Hello, there are 4 formatted arguments.");
}
```