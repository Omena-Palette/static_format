use static_format::const_format;

fn main() {
    let _out = const_format!("hello world with too many provided arguments {}", "hello", "world");
}
