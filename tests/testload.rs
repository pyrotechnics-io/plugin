use libloading::{Library, Symbol};

#[test]
fn load_lib() {
    unsafe {
        let lib = Library::new("/tmp/libplugin.so").unwrap();
        let func : Symbol::<extern fn(usize, usize) -> usize> = lib.get(b"add").unwrap();
        assert_eq!(func(3,2), 5);
    }
}
