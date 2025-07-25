use describer::Describe;
 
#[derive(Describe)]
#[prettify(hide_opt = true)]
struct MyStruct {
    maybe_vec_u8s: Option<Vec<u8>>,
    vec_maybe_u8s: Vec<Option<u8>>,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { vec_maybe_u8s: [u8]! }"
    );
}