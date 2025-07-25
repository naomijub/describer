use describer::Describe;
 
#[derive(Describe)]
struct MyStruct;
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct"
    );
}