use describer::Describe;
use std::collections::{BTreeMap, HashMap};
use indexmap::IndexMap;

#[derive(Describe)]
#[prettify(explicit_collections = true)]
struct MyExplicitStruct {
    map: HashMap<String, u8>,
    btree: BTreeMap<String, u8>,
    indexed: IndexMap<String, u8>
}

#[derive(Describe)]
struct MyImplicitStruct {
    map: HashMap<String, u8>,
    btree: BTreeMap<String, u8>,
    indexed: IndexMap<String, u8>
}
 
fn main() {
    assert_eq!(
        MyExplicitStruct::describe(),
        "MyExplicitStruct { map: HashMap<String!, u8!>!, btree: BTreeMap<String!, u8!>!, indexed: IndexMap<String!, u8!>! }"
    );
    assert_eq!(
        MyImplicitStruct::describe(),
        "MyImplicitStruct { map: {String!, u8!}!, btree: {String!, u8!}!, indexed: {String!, u8!}! }"
    );
}