use describer::Describe;
 
#[derive(Describe)]
struct MyStruct {
    opt: Option<bool>,
    my_string: String,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { opt: bool, my_string: String! }"
    );
}