# Describer

Helper crate that generates customizable Rust `Structs` describing strings.

## Installation
```toml
[dependencies]
describer = "0.1"
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
        "MyStruct {opt: bool, my_string: String!}"
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
        "MyStruct {maybe_vec_u8s: [u8!], vec_maybe_u8s: [u8]!, vec_u8s: [u8!]!, result: Result<u8!, String!>!}"
    );
}
```

## Limitations:
- TupleStructs are not yet supported.
- Structured Enum Variants are not yet supported.
- Tuple Enum Variants are not yet supported.
- Non derivable types are not supported.

## Inspired by:
- https://solana.com/developers/courses/program-optimization/rust-macros#example-of-a-procedural-macro
- https://medium.com/rust-masterclass/how-to-iterate-over-rust-struct-fields-65e7331a9b00
- https://github.com/evaporei/edn-derive/