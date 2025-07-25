use describer::Describe;

#[derive(Describe)]
#[prettify(tokens(separator = ",", spacing = "\n", keyval = "=>"))]
struct MyStruct {
    vec_u8s: Vec<u8>,
    other: u8,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct\n{\nvec_u8s=>[u8!]!,other=>u8!\n}"
    );
}