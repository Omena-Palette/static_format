use static_format::const_format;

macro_rules! some_macro {
    ($val:literal) => {$val};
}

fn main() {
    let out = const_format!("hello world with macro {}", some_macro!(5));
    assert_eq!(out, "hello world with macro 5");

    let a_lot_of_formats = const_format!(
        "{} hello {} world {} {} {} {}{}",
        some_macro!('.'), 5, 3, "hello", 1, 2, 3
    );
    assert_eq!(a_lot_of_formats, ". hello 5 world 3 hello 1 23")
}
