use std::ffi;

extern "C-unwind" {
    fn multiply(a: ffi::c_int, b: ffi::c_int, sheesh: *mut ffi::c_void) -> ffi::c_int;

    fn bad_jump_inside();
}

extern "C-unwind" fn weird_callback() {
    let d = Dummy::new();
    println!("calling bad_jump_inside");
    unsafe {
        bad_jump_inside();
    }
}

struct Dummy {}

impl Dummy {
    fn new() -> Self {
        println!("new dummy");
        Dummy {}
    }
}

impl Drop for Dummy {
    fn drop(&mut self) {
        println!("drop dummy");
    }
}

fn main() {
    println!("5 x 3 = {}", unsafe {
        multiply(5, 3, weird_callback as *mut _)
    });
}
