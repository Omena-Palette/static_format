#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/basic.rs");
    t.compile_fail("tests/ui/too_many_args.rs");
    t.compile_fail("tests/ui/missing_arg.rs");
    t.compile_fail("tests/ui/missing_args.rs");
}