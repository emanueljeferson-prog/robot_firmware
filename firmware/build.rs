fn main() {
    println!("cargo:rustc-link-search=native=src/services/rtos/build");
    println!("cargo:rustc-link-lib=static=freertos_interface");
    println!("cargo:rustc-link-search=native=src/services/RTOS/build");
    println!("cargo:rustc-link-lib=static=freertos_interface");
}