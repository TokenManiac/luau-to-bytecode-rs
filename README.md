
# luau-to-bytecode-rs

Exactly what the name is, compiles luau source to luau bytecode in rust.
I needed this for a side-project, use it or not or whatever.

## Usage

Usage is simple:

```rust
fn main() {
    let luau_source = r#"
        print("Hello bytecode!");
    "#;

    let compilation_result: Result<Vec<u8>, String> = luau_to_bytecode::compile_luau_to_bytecode( luau_source, Some(
        &mut luau_to_bytecode::LuaCompileOptions {
            optimizationLevel: 2,
            debugLevel: 2,
            ..Default::default()
        }
    ));

    match compilation_result {
        Ok(bytecode) => {
            println!("{bytecode:?}");
        }
        Err(error_msg) => {
            println!("Failed to compile! Error message: {:?}", error_msg);
        }
    }
}
```
