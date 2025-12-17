// somewhat-small luau -> bytecode in rust using the luau0-src crate without using the entirety of the mlua crate
// use it for whatever I just ⁿeed this for a project of mine

use std::ffi;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct LuaCompileOptions {
    pub optimizationLevel: i32,
    pub debugLevel: i32,
    pub typeInfoLevel: i32,
    pub coverageLevel: i32,
    pub vectorLib: *const ffi::c_char,
    pub vectorCtor: *const ffi::c_char,
    pub vectorType: *const ffi::c_char,
    pub mutableGlobals: *const *const ffi::c_char,
    pub userdataTypes: *const *const ffi::c_char,
    pub librariesWithKnownMembers: *const *const ffi::c_char,
    pub libraryMemberTypeCb:
        Option<extern "C" fn(library: *const ffi::c_char, member: *const ffi::c_char) -> i32>,
    pub libraryMemberConstantCb: Option<
        extern "C" fn(
            library: *const ffi::c_char,
            member: *const ffi::c_char,
            constant: *mut LuaCompileConstant,
        ),
    >,
    pub disabledBuiltins: *const *const ffi::c_char,
}
pub type LuaCompileConstant = *mut ffi::c_void;

impl Default for LuaCompileOptions {
    fn default() -> Self {
        Self {
            optimizationLevel: 1,
            debugLevel: 1,
            typeInfoLevel: 0,
            coverageLevel: 0,
            vectorLib: std::ptr::null(),
            vectorCtor: std::ptr::null(),
            vectorType: std::ptr::null(),
            mutableGlobals: std::ptr::null(),
            userdataTypes: std::ptr::null(),
            librariesWithKnownMembers: std::ptr::null(),
            libraryMemberTypeCb: None,
            libraryMemberConstantCb: None,
            disabledBuiltins: std::ptr::null(),
        }
    }
}

// todo: maybe change the result just Vec<u8> and panic when bytecode_ptr is null?
pub fn compile_luau_to_bytecode(
    source: &str,
    options: Option<&mut LuaCompileOptions>,
) -> Result<Vec<u8>, String> {
    let mut outsize = 0usize;
    let options_ptr = options.map_or(std::ptr::null_mut(), |options| {
        options as *mut LuaCompileOptions
    });

    let bytecode_ptr = unsafe {
        luau_compile(
            source.as_ptr().cast::<ffi::c_char>(),
            source.len(),
            options_ptr,
            &mut outsize,
        )
    };

    if bytecode_ptr.is_null() {
        // maybe panic here instead?
        return Err("luau_compile returned null".to_string());
    }

    let bytecode =
        unsafe { std::slice::from_raw_parts(bytecode_ptr.cast::<u8>(), outsize).to_vec() };
    unsafe { free(bytecode_ptr.cast::<ffi::c_void>()) };

    // The first byte being null means there was an error compiling, the error message is attached.
    if bytecode[0] == 0 {
        if bytecode.len() <= 2 {
            return Err("Unknown compilation error.".to_string());
        }

        // maybe use an array instead of losing utf8 chars?
        return Err(String::from_utf8_lossy(&bytecode[1..]).to_string());
    }

    Ok(bytecode)
}

unsafe extern "C" {
    fn luau_compile(
        source: *const ffi::c_char,
        size: usize,
        options: *mut LuaCompileOptions,
        outsize: *mut usize,
    ) -> *mut ffi::c_char;

    fn free(ptr: *mut ffi::c_void);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compile() {
        let valid_code = r#"
            print("Hello from Luau!")
        "#;
        let compile_result = compile_luau_to_bytecode(valid_code, None);

        if let Err(err_msg) = &compile_result {
            println!("{}", err_msg);
        }

        assert!(compile_result.is_ok());
    }

    #[test]
    fn should_fail_on_invalid_code() {
        let invalid_code = r#"
            this is not valid luau code!
        "#;
        let compile_result = compile_luau_to_bytecode(invalid_code, None);

        assert!(compile_result.is_err());
    }

    #[test]
    fn should_give_valid_error_message() {
        let invalid_code = r#"
           print(""") 
        "#;
        let compile_result = compile_luau_to_bytecode(invalid_code, None);

        if let Err(err_msg) = &compile_result {
            assert!(err_msg.contains("malformed string"));
        } else {
            panic!("Expected an error but got success.");
        }
    }

    #[test]
    fn table_driven_tests() {
        let test_cases: Vec<(&str, bool)> = vec![
            (
                r#"print("Test 1")"#,
                true, // should compile
            ),
            (
                r#"function foo() return 42 end"#,
                true, // should compile
            ),
            (
                r#"local ab = {if true then 16 end}"#,
                false, // should not compile
            ),
        ];

        for (source, should_compile) in test_cases {
            let result = compile_luau_to_bytecode(source, None);

            assert_eq!(
                result.is_ok(),
                should_compile,
                "Source: {}\nExpected to compile: {}, but got: {:?}",
                source,
                should_compile,
                result
            );
        }
    }
}
