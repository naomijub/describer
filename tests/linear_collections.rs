use describer::Describe;
use std::collections::{BTreeSet, HashSet};
use indexmap::IndexSet;

#[derive(Describe)]
#[prettify(explicit_collections = true)]
#[prettify(hide_name = true)]
struct MyExplicitStruct {
    vec_u8s: Vec<u8>,
    set: HashSet<u8>,
    btree: BTreeSet<u8>,
    indexed: IndexSet<u8>
}

#[derive(Describe)]
struct MyImplicitStruct {
    vec_u8s: Vec<u8>,
    set: HashSet<u8>,
    btree: BTreeSet<u8>,
    indexed: IndexSet<u8>
}
 
fn main() {
    assert_eq!(
        MyExplicitStruct::describe(),
        "{ vec_u8s: Vec<u8!>!, set: HashSet<u8!>!, btree: BTreeSet<u8!>!, indexed: IndexSet<u8!>! }"
    );
    assert_eq!(
        MyImplicitStruct::describe(),
        "MyImplicitStruct { vec_u8s: [u8!]!, set: [u8!]!, btree: [u8!]!, indexed: [u8!]! }"
    );
}