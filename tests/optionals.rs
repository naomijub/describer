use describer::Describe;
 
#[derive(Describe)]
struct MyStruct {
    maybe_vec_u8s: Option<Vec<u8>>,
    vec_maybe_u8s: Vec<Option<u8>>,
    vec_u8s: Vec<u8>,
    result: Result<u8, String>,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct {maybe_vec_u8s: [u8!], vec_maybe_u8s: [u8]!, vec_u8s: [u8!]!, result: Result<u8!, String!>!}"
    );
}