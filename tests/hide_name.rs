use describer::Describe;
 
#[derive(Describe)]
#[prettify(hide_name = true)]
struct MyStruct {
    opt: Option<bool>,
    my_string: String,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "{ opt: bool, my_string: String! }"
    );
}