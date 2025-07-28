# Describer

Helper crate that generates customizable Rust `Structs` describing strings.

## Installation
```toml
[dependencies]
describer = "0.2"
```

## Usage

> Original syntax is inspired by GraphQL, reason why required fields end with `!`.

### Basic Structs:

Simple struct types will represent `field_name: T` as `field_name: T!`, while optional fields will represent `optional_field_name: Option<T>` as `optional_field_name: T`. Struct will be represented as `StructName { field_name: T, .. }`.
```rust
use describer::Describe;
 
#[derive(Describe)]
struct MyStruct {
    // By default, optional fields will be present and just represent the generic type, `bool`
    opt: Option<bool>,
    // By default, required fields will end with a `!`, `String!`
    my_string: String,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { opt: bool, my_string: String! }"
    );
}
```

### Unit Structs:

Unit struct will be represented just by the struct name, `StructName`:
```rust
use describer::Describe;
 
#[derive(Describe)]
struct MyStruct;
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct"
    );
}
```

### Enum types:

Enum variants are represented in a traditional Set like representation inside a `#{ <variants> }`, so `EnumName #{ VariantA, VariantB, .. }`
```rust
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
```

### Results and Vecs:

Results are explicitly represented, with `Result<OK, ERROR>`, while `Vec`s are represented, by default, inside `[T]`
```rust
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
        "MyStruct { maybe_vec_u8s: [u8!], vec_maybe_u8s: [u8]!, vec_u8s: [u8!]!, result: Result<u8!, String!>! }"
    );
}
```

## Prettify Output

### Hiding optional fields:

To hide optional fields you can add the attribute `#[prettify(hide_opt = true)]`. It only hides root field optionals:

```rust
use describer::Describe;
 
#[derive(Describe)]
#[prettify(hide_opt = true)]
struct MyStruct {
    maybe_vec_u8s: Option<Vec<u8>>,
    vec_maybe_u8s: Vec<Option<u8>>,
    vec_u8s: Vec<u8>,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { vec_maybe_u8s: [u8]!, vec_u8s: [u8!]! }"
    );
}
```

### Explicit Vec and linear collections:

> Linear collections: `Vec<T>, HashSet<T>, BTreeSet<T>, indexmap::IndexSet<T>`

To show linear collections explicit type `#[prettify(explicit_collections = true)]`. By default they are `[T]`:

```rust
use describer::Describe;
 
#[derive(Describe)]
#[prettify(explicit_collections = true)]
struct MyStruct {
    vec_u8s: Vec<u8>,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct { vec_u8s: Vec<u8!>! }"
    );
}
```

### Explicit key-value collections:

> Key-value collections: `HashMap<K, T>, BTreeMap<K, T>, indexmap::IndexMap<K, T>`

To show key-value collections explicit type `#[prettify(explicit_collections = true)]`. By default they are `{K, T}`:

```rust
use describer::Describe;
use std::collections::{BTreeMap, HashMap};
use indexmap::IndexMap;

#[derive(Describe)]
#[prettify(explicit_collections = true)]
struct MyExplicitStruct {
    map: HashMap<String, u8>
}

#[derive(Describe)]
struct MyImplicitStruct {
    map: HashMap<String, u8>,
}
 
fn main() {
    assert_eq!(
        MyExplicitStruct::describe(),
        "MyExplicitStruct { map: HashMap<String!, u8!>! }"
    );
    assert_eq!(
        MyImplicitStruct::describe(),
        "MyImplicitStruct { map: {String!, u8!}! }"
    );
}
```

### Custom Separators and Spacing:

The default separator is `", "` , the default spacing is `" "` and the default key-value (`keyval`) separator is `": "` with `tokens` it is possible to change separator and spacing to the desired customization.

```rust
use describer::Describe;

#[derive(Describe)]
#[prettify(tokens(separator = " && ", spacing = "\n", keyval = "=>"))]
struct MyStruct {
    vec_u8s: Vec<u8>,
    other: u8,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct\n{\nvec_u8s=>[u8!]! && other=>u8!\n}"
    );
}
```

### Hide Struct/Enum Name

To hide struct or enum name you can add the attribute `#[prettify(hide_name = true)]`:

```rust
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
```

### Multiple `prettify`
For now, it is necessary to add multiple `#[prettify(...)]` attributes:

```rust
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
```

## Limitations:
- TupleStructs are not yet supported.
- Structured Enum Variants are not yet supported.
- Tuple Enum Variants are not yet supported.
- Non derivable types are not supported.
- Nested describing is not yet supported.

## Inspired by:
- https://solana.com/developers/courses/program-optimization/rust-macros#example-of-a-procedural-macro
- https://medium.com/rust-masterclass/how-to-iterate-over-rust-struct-fields-65e7331a9b00
- https://github.com/evaporei/edn-derive/