fn noop() {
    println!("cqll noop");
}

fn main() {
    let fn_ptr = noop as usize;
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x{:x}", fn_ptr);
    println!("noop as *cont T:{:p}", typed_fn_ptr);

    unsafe {
        let typed_fn: fn() -> () = std::mem::transmute(typed_fn_ptr);
        typed_fn();
    }
}
