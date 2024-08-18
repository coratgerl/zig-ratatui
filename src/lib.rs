#[no_mangle]
extern "C" fn hello_from_rust() {
    println!("Hello from {}", "rust");
}
