# Describer

Helper crate that generates customizable Rust `Structs` describing strings.

## Installation
```toml
[dependencies]
describer = "0.1"
```

## Usage

```rust
use describer::Describe;
 
#[derive(Describe)]
struct MyStruct {
    opt: Option<bool>,
    my_string: String,
}
 
fn main() {
    assert_eq!(
        MyStruct::describe(),
        "MyStruct {opt: bool, my_string: String!}"
    );
}
```

Inspired by:
- https://solana.com/developers/courses/program-optimization/rust-macros#example-of-a-procedural-macro
- https://medium.com/rust-masterclass/how-to-iterate-over-rust-struct-fields-65e7331a9b00
- https://github.com/evaporei/edn-derive/