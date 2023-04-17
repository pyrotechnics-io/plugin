fn load_lib() -> usize {
    unsafe {
        let lib = libloading::Library::new("/tmp/libplugin.so").unwrap();
        let func : libloading::Symbol::<extern fn(usize, usize) -> usize> = lib.get(b"add").unwrap();
        func(5, 8)
    }
}

pub fn main() {
    let result = load_lib();
    println!("Result is {}", result);
}