extern crate cc;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./aarch64.c");
    
    let target = std::env::var("TARGET").unwrap();
    
    if target.contains("aarch64") {
        cc::Build::new()
            .file("./aarch64.c")
            .compile("libneon");
    }
    
    Ok(())
}