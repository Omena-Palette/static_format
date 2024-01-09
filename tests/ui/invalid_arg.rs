use static_format::const_format;

fn hello() -> u8 {
    5
}

fn main() {
    const_format!("{}", hello());
}