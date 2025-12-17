fn main() {
    let luau_source_code = r#"
        local tbl = {
            "Hello",
            "world!"
        }
        local format_string = table.concat(tbl, ", ")
        print(format_string)
    "#;

    let compile_result = luau_to_bytecode::compile_luau_to_bytecode(
        luau_source_code,
        None,
    );

    match compile_result {
        Ok(bytecode) => {
            println!("Luau bytecode ({} bytes):", bytecode.len());
            for (idx, byte) in bytecode.iter().enumerate() {
                print!(
                    "{byte:02x}{}",
                    if (idx + 1) % 16 == 0 { "\n" } else { " " }
                );
            }
            if bytecode.len() % 16 != 0 {
                println!();
            }
        }
        Err(err_msg) => {
            panic!("Failed to compile Luau source.\nError:{}", err_msg);
        }
    }
}
