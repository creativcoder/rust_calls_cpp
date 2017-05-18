
use std::process::Command;

const LIB_NAME: &'static str = "magic";
const LIB_PREFIX: &'static str = "lib";

fn main() {
    // Compile  our object file
	let _build_o = Command::new("g++").args(&["-Wall",
                                             "-fPIC",
                                             "-std=c++14",
                                             "-c",
                                             &format!("{}/{}.cpp", LIB_PREFIX, LIB_NAME),
                                             "-o",
                                             &format!("{}/{}.o", LIB_PREFIX, LIB_NAME)])
            						 .output()
            						 .expect("failed to build object file");
    // Build our shared object file
    let _build_so = Command::new("ld").args(&["-shared",
                                             &format!("{}/{}.o", LIB_PREFIX, LIB_NAME),
                                             "-o",
                                             &format!("{}/lib{}.so", LIB_PREFIX, LIB_NAME)])
    								 .output()
    								 .expect("failed to build shared object file");

    // Tell rustc we are building a so object
    println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);
    // Specifiy our shared library search path
    println!("cargo:rustc-link-search=native={}", LIB_PREFIX);
}
