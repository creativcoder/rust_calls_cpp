#![feature(libc)]
#![feature(link_args)]

extern crate libc;
use libc::size_t;

#[link(name = "magic")]
// Used for customizing the way linker links
#[link_args = "-lstdc++ -Wl,-rpath,lib"]
extern {
    fn add(a: size_t, b: size_t) -> size_t;
}

fn main() {
    unsafe {
        println!("Result from add: {}", add(1, 2));
    }
}