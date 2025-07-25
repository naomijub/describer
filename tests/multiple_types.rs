use describer::Describe;
 
#[derive(Describe)]
struct MyStruct {
    b1: Option<bool>,
    b2: bool,
    n1: i64,
    n2: Option<i64>,
    s1: String,
    s2: Option<String>,
    c1: char,
    c2: Option<char>
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { b1: bool, b2: bool!, n1: i64!, n2: i64, s1: String!, s2: String, c1: char!, c2: char }"
    );
}