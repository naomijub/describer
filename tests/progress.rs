#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple.rs");
    t.pass("tests/multiple_types.rs");
    t.compile_fail("tests/err_tuple_struct.rs");
}
