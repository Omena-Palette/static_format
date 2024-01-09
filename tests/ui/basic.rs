use static_format::const_format;

macro_rules! some_macro {
    ($val:literal) => {$val};
}

fn main() {
    let out = const_format!("hello world with macro {} burgers", some_macro!(5));
    dbg!(out);
}
