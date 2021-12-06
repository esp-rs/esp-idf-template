{% unless std %}#![no_std]

{% endunless %}use esp_idf_sys as _; // If using the `libstart` feature of `esp-idf-sys`, always keep this module imported

{% unless std %}use log::*;

{% endunless %}#[no_mangle]
extern "C" fn rust_main() -> i32 {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
{% if std %}
    println!("Hello world from Rust!");{% else %}
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello world from Rust!");{% endif %}

    42
}
