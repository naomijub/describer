#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple.rs");
    t.pass("tests/multiple_types.rs");
    t.pass("tests/unit.rs");
    t.pass("tests/simple_enum.rs");
    t.compile_fail("tests/err_tuple_struct.rs");
    t.pass("tests/optionals.rs");
}
