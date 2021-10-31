{% unless std %}#![no_std]
#![no_main]

{% endunless %}#[allow(unused_imports)]
use esp_idf_sys; // If using the `binstart` feature of `esp-idf-sys`, always keep the module imported
{% unless std %}use esp_idf_svc;

use log::*;{% endunless %}

{% unless std %}#[no_mangle]
{% endunless %}fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

{% if std %}
    println!("Hello, world!");{% else %}
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");{% endif %}
}
