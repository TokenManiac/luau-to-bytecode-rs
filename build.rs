fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    let artifacts = luau0_src::Build::new().build();
    artifacts.print_cargo_metadata();
}