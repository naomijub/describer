use describer::Describe;
 
#[derive(Describe)]
enum MyEnum {
    VarA,
    VarB
}
 
fn main() {
    assert_eq!(
        MyEnum::describe(),
        "MyEnum #{ VarA, VarB }"
    );
}