use describer::Describe;

#[derive(Describe)]
#[prettify(token(separator = ",", spacing = "\n", keyval = "=>"))]
#[prettify(tokens(sepurator = ",", spacigg = "\n", key_val = "=>"))]
#[prettify(hide_opt = 1)]
#[prettify(explicit_collections = "true")]
#[prettify(hide_optional = true)]
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