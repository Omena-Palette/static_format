use static_format::const_format;

fn main() {
    let _out = const_format!("hello world missing arguments {} and {}", 5);
}
