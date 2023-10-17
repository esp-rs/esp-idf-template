{% unless std -%}
#![no_std]
{% endunless -%}

#[no_mangle]
extern "C" fn rust_main() -> i32 {
{%- if hal %}
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
{%- elsif std %}
    println!("Hello, world from Rust!");
{%- endif %}
    42
}

{%- if not hal %}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
{%- endif %}
